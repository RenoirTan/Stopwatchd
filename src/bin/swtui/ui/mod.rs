pub mod bar;
pub mod border;
pub mod color;
pub mod focus_panel;
pub mod geometry;
pub mod list_panel;
pub mod prompt;

use std::{
    sync::Arc,
    path::PathBuf
};

use stopwatchd::{
    communication::{
        client::{Request, receive_reply_bytes},
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
    geometry::{Size, BordersGeometry, BarLocation},
    list_panel::{ListPanel, ListPanelState},
    focus_panel::{FocusPanel, FocusPanelState},
    prompt::{Prompt, PromptState}
};

pub struct Ui {
    pub window: Arc<pancurses::Window>,
    pub border: Border,
    pub list_panel: ListPanel,
    pub list_panel_state: ListPanelState,
    pub focus_panel: FocusPanel,
    pub focus_panel_state: FocusPanelState,
    pub prompt: Prompt,
    pub prompt_state: PromptState,
    pub bar: Bar,
    focus_active: bool,
    pub formatter: Formatter,
    pub ssock_path: PathBuf
}

impl Ui {
    pub fn new(
        window: Arc<pancurses::Window>,
        border: Border,
        list_panel_state: ListPanelState,
        focus_panel_state: FocusPanelState,
        prompt_state: PromptState,
        bar: Bar,
        focus_active: bool,
        formatter: Formatter,
        ssock_path: PathBuf
    ) -> Self {
        window.nodelay(false);
        window.keypad(true);
        pancurses::noecho();
        let g = BordersGeometry::from_window(&window);
        let list_panel = ListPanel::new(Arc::new(ListPanel::newwin(g)));
        let focus_panel = FocusPanel::new(Arc::new(FocusPanel::newwin(g)));
        let prompt = Prompt::new(Arc::new(Prompt::newwin()));
        Self {
            window,
            border,
            list_panel,
            list_panel_state,
            focus_panel,
            focus_panel_state,
            prompt,
            prompt_state,
            bar,
            focus_active,
            formatter,
            ssock_path
        }
    }

    pub async fn refresh_list(&mut self) {
        let request = Request::info_all(false);
        let ssock_path_str = self.ssock_path.display();

        // We must constantly reconnect because `swd` drops the other end
        // of the line once it has replied the first time and [`UnixListener`]
        // has to be triggered again.
        trace!("[swtui::ui::Ui::refresh_list] connecting to {}", ssock_path_str);
        // TODO: show separate messages depending on known errors
        let stream = request.send_to_socket(&self.ssock_path).await
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

    // TODO: when refreshing a stopwatch, the access order changes and mucks
    // everything up
    pub async fn refresh_stopwatch(&mut self) {
        if self.focus_panel_state.selected.is_none() { return; }
        let identifier = self.focus_panel_state.selected.as_ref().unwrap();
        let raw_id = identifier.to_string();
        let request = Request::info_some(vec![raw_id.clone()], true);
        let ssock_path_str = self.ssock_path.display();

        trace!("[swtui::ui::Ui::refresh_stopwatch] connecting to {}", ssock_path_str);
        // TODO: show separate messages depending on known errors
        let stream = request.send_to_socket(&self.ssock_path).await
            .expect(&format!("could not send request to {}", ssock_path_str));

        info!("[swtui::ui::Ui::refresh_stopwatch] reading response from server");
        let braw = receive_reply_bytes(stream).await
            .expect(&format!("could not read reply message from {}", ssock_path_str));

        let mut reply = Reply::from_bytes(&braw)
            .expect(&format!("could not convert message to reply"));

        if !reply.errors.is_empty() {
            panic!("reply to stopwatch refresh request returns error, requires reworking");
        }
        if let SpecificAnswer::Info(InfoAnswer::Basic) = reply.specific_answer {
            self.focus_panel_state.details = reply.successful.remove(&raw_id);
        } else {
            panic!("refresh request should be replied with InfoAnswer::All");
        }
    }

    pub fn draw(&self) {
        self.bar.draw(self, self.focus_active);
        self.window.refresh(); // must be placed here or the other windows will be cleared
        self.list_panel.draw(self);
        self.list_panel.refresh();
        self.focus_panel.draw(self);
        self.focus_panel.refresh();
        if self.prompt_state.visible {
            self.prompt.draw(self);
            self.prompt.refresh();
        }
    }

    /// (rows, columns) or (y, x)
    pub fn dimensions(&self) -> Size {
        Size::window_dimensions(&self.window)
    }

    pub fn borders_geometry(&self) -> BordersGeometry {
        BordersGeometry::from_window(&self.window)
    }

    pub fn bar_location(&self) -> BarLocation {
        let Size { x: _x, y } = self.dimensions();
        BarLocation { y: y-1 }
    }

    pub fn add_string<S: AsRef<str>>(&self, x: i32, y: i32, s: S) -> i32 {
        self.window.mvaddstr(y, x, &s);
        x + s.as_ref().len() as i32
    }

    pub fn scroll(&mut self, up: bool) {
        let height = self.list_panel.height();
        self.list_panel_state.scroll_inner(up, height as usize);
    }

    pub fn scroll_home(&mut self) {
        let height = self.list_panel.height();
        self.list_panel_state.scroll_home(height as usize);
    }

    pub fn scroll_end(&mut self) {
        let height = self.list_panel.height();
        self.list_panel_state.scroll_end(height as usize);
    }

    pub async fn set_focus_active(&mut self, yes: bool) {
        self.focus_active = yes;
        if yes {
            self.focus_panel_state.selected = self.list_panel_state.identifiers
                .get(self.list_panel_state.selected)
                .cloned();
            match self.focus_panel_state.selected {
                Some(ref _id) => self.refresh_stopwatch().await,
                None => self.focus_panel_state.details = None
            }
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

    pub fn prompt_name(&mut self) {
        self.prompt_state.visible = true;
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
            ListPanelState::default(),
            FocusPanelState::default(),
            PromptState::default(),
            Bar,
            false,
            Formatter::default(),
            PathBuf::new()
        )
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
