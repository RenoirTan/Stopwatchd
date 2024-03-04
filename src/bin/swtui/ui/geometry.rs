use std::cmp::{max, min};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Size {
    pub x: i32,
    pub y: i32
}

impl Size {
    pub fn window_dimensions(window: &pancurses::Window) -> Self {
        let (y, x) = window.get_max_yx();
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Location {
    pub x: i32,
    pub y: i32
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Rectangle {
    pub loc: Location,
    pub size: Size
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct BordersGeometry {
    pub top_left: Location,
    pub bottom_right: Location,
    pub focus_x: i32
}

impl BordersGeometry {
    pub fn calculate(size: Size) -> Self {
        let Size { x, y } = size;
        BordersGeometry {
            top_left: Location { x: 0, y: 0 },
            // Leave 1 line at the bottom for bar
            bottom_right: Location { x: x-1, y: y-2 },
            // 21 <= x <= 49
            focus_x: min(max(x/3, 21), 49)
        }
    }

    pub fn from_window(window: &pancurses::Window) -> Self {
        Self::calculate(Size::window_dimensions(window))
    }

    pub fn list_panel_geometry(&self) -> (i32, i32, i32, i32) {
        // minimum size of list panel guaranteed
        let left = self.top_left.x + 1;
        let right = self.focus_x - 1;
        let top = self.top_left.y + 1;
        let bottom = self.bottom_right.y - 1;
        (left, right, top, bottom)
    }

    pub fn focus_panel_geometry(&self) -> (i32, i32, i32, i32) {
        let left = self.focus_x + 1;
        let right = self.bottom_right.x - 1;
        let top = self.top_left.y + 1;
        let bottom = self.bottom_right.y - 1;
        (left, right, top, bottom)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct BarLocation {
    pub y: i32
}
