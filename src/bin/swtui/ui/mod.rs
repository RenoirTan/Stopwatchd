pub mod bar;
pub mod border;
pub mod color;
pub mod focus_panel;
pub mod geometry;
pub mod list_panel;

use std::{
    cmp::{max, min},
    sync::Arc
};

use stopwatchd::communication::details::StopwatchDetails;

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
    focus_active: bool
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
        focus_active: bool
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
            focus_active
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
            false
        )
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
