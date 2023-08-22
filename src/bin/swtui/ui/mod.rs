pub mod bar;
pub mod border;
pub mod color;
pub mod geometry;
pub mod list_panel;

use std::{
    cmp::{max, min},
    sync::Arc
};

use self::{
    bar::Bar,
    border::Border,
    geometry::{Size, Location, BordersGeometry, BarLocation},
    list_panel::{ListPanel, ListPanelState}
};

pub struct Ui {
    pub window: Arc<pancurses::Window>,
    pub border: Border,
    pub list_panel: ListPanel,
    pub list_panel_state: ListPanelState,
    pub bar: Bar,
    pub focus_active: bool
}

impl Ui {
    pub fn new(
        window: Arc<pancurses::Window>,
        border: Border,
        list_panel: ListPanel,
        list_panel_state: ListPanelState,
        bar: Bar,
        focus_active: bool
    ) -> Self {
        window.nodelay(false);
        window.keypad(true);
        pancurses::noecho();
        Self { window, border, list_panel, list_panel_state, bar, focus_active }
    }

    pub fn draw(&self) {
        self.border.draw(self, self.focus_active);
        self.list_panel.draw(self, &self.list_panel_state);
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
