#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Size {
    pub x: i32,
    pub y: i32
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
    pub separator_x: i32
}

impl BordersGeometry {
    pub fn list_panel_geometry(&self) -> (i32, i32, i32, i32) {
        // minimum size of list panel guaranteed
        let left = self.top_left.x + 1;
        let right = self.separator_x - 1;
        let top = self.top_left.y + 1;
        let bottom = self.bottom_right.y - 1;
        (left, right, top, bottom)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct BarLocation {
    pub y: i32
}
