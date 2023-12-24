pub mod bar;
pub mod border;
pub mod color;
pub mod focus_panel;
pub mod geometry;
pub mod list_panel;

use std::{
    cmp::{max, min},
    sync::Arc,
    path::Path
};

use stopwatchd::{
    communication::{
        details::StopwatchDetails,
        client::{CommonArgs, Request, receive_reply_bytes},
        request_specifics::{SpecificArgs, InfoArgs},
        server::Reply,
        reply_specifics::{SpecificAnswer, InfoAnswer}
    },
    fmt::Formatter,
    models::stopwatch::State,
    traits::Codecable
};

use self::{
    bar::Bar,
    border::Border,
    geometry::{Size, Location, BordersGeometry, BarLocation},
    list_panel::{ListPanel, ListPanelState},
    focus_panel::{FocusPanel, FocusPanelState}
};

pub struct Ui {
    pub window: Arc<pancurses::Window>,
    pub border: Border,
    pub list_panel: ListPanel,
    pub list_panel_state: ListPanelState,
    pub focus_panel: FocusPanel,
    pub focus_panel_state: FocusPanelState,
    pub bar: Bar,
    focus_active: bool,
    pub formatter: Formatter
}

impl Ui {
    pub fn new(
        window: Arc<pancurses::Window>,
        border: Border,
        list_panel: ListPanel,
        list_panel_state: ListPanelState,
        focus_panel: FocusPanel,
        focus_panel_state: FocusPanelState,
        bar: Bar,
        focus_active: bool,
        formatter: Formatter
    ) -> Self {
        window.nodelay(false);
        window.keypad(true);
        pancurses::noecho();
        Self {
            window,
            border,
            list_panel,
            list_panel_state,
            focus_panel,
            focus_panel_state,
            bar,
            focus_active,
            formatter
        }
    }

    pub async fn refresh_list<P: AsRef<Path>>(&mut self, ssock_path: P) {
        let common_args = CommonArgs::default();
        let specific_args = SpecificArgs::Info(InfoArgs);
        let request = Request::new(common_args, specific_args);
        let ssock_path_str = ssock_path.as_ref().display();

        // We must constantly reconnect because `swd` drops the other end
        // of the line once it has replied the first time and [`UnixListener`]
        // has to be triggered again.
        trace!("connecting to {}", ssock_path_str);
        // TODO: show separate messages depending on known errors
        let stream = request.send_to_socket(&ssock_path).await
            .expect(&format!("could not send request to {}", ssock_path_str));

        info!("[swtui::ui::Ui::refresh_list] reading response from server");
        let braw = receive_reply_bytes(stream).await
            .expect(&format!("could not read reply message from {}", ssock_path_str));

        let reply = Reply::from_bytes(&braw)
            .expect(&format!("could not convert message to reply"));

        if !reply.errors.is_empty() {
            panic!("reply to refresh request returns error, requires reworking");
        }
        if let SpecificAnswer::Info(InfoAnswer::All(ref all)) = reply.specific_answer {
            self.list_panel_state.identifiers = all.access_order
                .iter()
                .map(|s| reply.successful.get(s).unwrap().identifier.clone())
                .collect();
        } else {
            panic!("refresh request should be replied with InfoAnswer::All");
        }
    }

    pub fn draw(&self) {
        self.border.draw(self, self.focus_active);
        self.list_panel.draw(self, &self.list_panel_state);
        self.focus_panel.draw(self, &self.focus_panel_state);
        self.bar.draw(self, self.focus_active);
        self.window.refresh();
    }

    /// (rows, columns) or (y, x)
    pub fn dimensions(&self) -> Size {
        let (y, x) = self.window.get_max_yx();
        Size { x, y }
    }

    pub fn borders_geometry(&self) -> BordersGeometry {
        let Size { x, y } = self.dimensions();
        BordersGeometry {
            top_left: Location { x: 0, y: 0 },
            // Leave 1 line at the bottom for bar
            bottom_right: Location { x: x-1, y: y-2 },
            // 21 <= x <= 49
            separator_x: min(max(x/3, 21), 49)
        }
    }

    pub fn bar_location(&self) -> BarLocation {
        let Size { x: _x, y } = self.dimensions();
        BarLocation { y: y-1 }
    }

    pub fn add_string<S: AsRef<str>>(&self, x: i32, y: i32, s: S) -> i32 {
        self.window.mvaddstr(y, x, &s);
        x + s.as_ref().len() as i32
    }

    fn list_panel_height(&self) -> i32 {
        let (_l, _r, top, bottom) = self.borders_geometry().list_panel_geometry();
        bottom - top + 1
    }

    pub fn scroll(&mut self, up: bool) {
        let height = self.list_panel_height();
        self.list_panel_state.scroll_inner(up, height as usize);
    }

    pub fn scroll_home(&mut self) {
        let height = self.list_panel_height();
        self.list_panel_state.scroll_home(height as usize);
    }

    pub fn scroll_end(&mut self) {
        let height = self.list_panel_height();
        self.list_panel_state.scroll_end(height as usize);
    }

    pub fn set_focus_active(&mut self, yes: bool) {
        self.focus_active = yes;
        if yes {
            self.focus_panel_state.selected = self.list_panel_state.identifiers
                .get(self.list_panel_state.selected)
                .cloned();
            self.focus_panel_state.details = match self.focus_panel_state.selected {
                Some(ref id) => Some(StopwatchDetails::dummy(id.clone())),
                None => None
            };
        }
    }

    pub fn is_focus_active(&self) -> bool {
        self.focus_active
    }

    pub fn toggle_state(&mut self) {
        // TODO: Send command to swd
        if let Some(ref mut d) = self.focus_panel_state.details {
            match d.state {
                State::Playing => d.state = State::Paused,
                State::Paused => d.state = State::Playing,
                State::Ended => {}
            }
        }
    }
}

impl AsRef<pancurses::Window> for Ui {
    fn as_ref(&self) -> &pancurses::Window {
        &self.window
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new(
            Arc::new(pancurses::initscr()),
            Border::new_unicode(),
            ListPanel,
            ListPanelState::default(),
            FocusPanel,
            FocusPanelState::default(),
            Bar,
            false,
            Formatter::default()
        )
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
