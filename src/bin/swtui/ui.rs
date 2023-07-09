pub struct Ui {
    pub window: pancurses::Window
}

impl Ui {
    pub fn new(window: pancurses::Window) -> Self {
        Self { window }
    }

    pub fn reset(&self) {
        self.window.printw("hello");
        self.window.refresh();
    }

    /// (rows, columns) or (y, x)
    pub fn dimensions(&self) -> (i32, i32) {
        self.window.get_max_yx()
    }

    /// (top-left y, top-left x, bottom-right y, bottom-right x)
    pub fn catalog_panel_geometry(&self) -> (i32, i32, i32, i32) {
        let (wy, wx) = self.dimensions();
        (0, 0, wy, wx/3)
    }

    /// (top-left y, top-left x, bottom-right y, bottom-right x)
    pub fn focus_panel_geometry(&self) -> (i32, i32, i32, i32) {
        let (wy, wx) = self.dimensions();
        (0, (wx/3)+2, wy, wx)
    }

    pub fn separator_x(&self) -> i32 {
        let (_wy, wx) = self.dimensions();
        (wx/3)+1
    }
}

impl AsRef<pancurses::Window> for Ui {
    fn as_ref(&self) -> &pancurses::Window {
        &self.window
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new(pancurses::initscr())
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
