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
