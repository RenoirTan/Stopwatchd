pub mod border;
pub mod color;
pub mod geometry;

use std::cmp::{max, min};
use self::{
    border::Border,
    geometry::{Size, Location, BordersGeometry, BarLocation}
};

pub struct Ui {
    pub window: pancurses::Window,
    pub border: Border
}

impl Ui {
    pub fn new(window: pancurses::Window, border: Border) -> Self {
        Self { window, border }
    }

    pub fn reset(&self) {
        self.border.draw(self, false);
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
}

impl AsRef<pancurses::Window> for Ui {
    fn as_ref(&self) -> &pancurses::Window {
        &self.window
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new(pancurses::initscr(), Border::new_unicode())
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
