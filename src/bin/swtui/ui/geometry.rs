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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct BarLocation {
    pub y: i32
}
