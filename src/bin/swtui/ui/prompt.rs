use std::{
    cmp::{min, max},
    sync::Arc
};

use crate::ui::{Ui, color::ColorPair};

pub struct Prompt {
    pub window: Arc<pancurses::Window>
}

impl Prompt {
    pub fn newwin(main: &pancurses::Window) -> pancurses::Window {
        let rows = 4;
        let max_x = main.get_max_x();
        let columns = min(max(max_x/2, 16), max_x); // 16 <= columns <= max_x
        let beg_y = (main.get_max_y() - rows) / 2;
        let beg_x = (max_x - columns) / 2;
        main.subwin(rows, columns, beg_y, beg_x).unwrap()
    }

    pub fn new(window: Arc<pancurses::Window>) -> Self {
        Self { window }
    }

    pub fn geometry(&self) -> (i32, i32, i32, i32) {
        let (max_y, max_x) = self.window.get_max_yx();
        (1, max_x-1, 1, max_y-1)
    }

    pub fn clear(&self) {
        let (left, right, top, bottom) = self.geometry();
        ColorPair::Active.set_color(&self.window, false);
        for x in left..=right {
            for y in top..=bottom {
                self.window.mvaddch(y, x, ' ');
            }
        }
    }

    pub fn border(&self, ui: &Ui) {
        ui.border.border(&self.window, ColorPair::Selected, false);
    }

    pub fn draw(&self, ui: &Ui) {
        self.clear();
        self.border(ui);
        self.window.mvaddstr(1, 1, "Name for stopwatch:");
        let length = ui.prompt_state.name.len();
        let (left, right, _top, _bottom) = self.geometry();
        let max_displayed_len = (right - left) as usize;
        let displayed = if length > max_displayed_len {
            &ui.prompt_state.name[length-max_displayed_len..]
        } else {
            &ui.prompt_state.name
        };
        self.window.mvaddnstr(2, 1, displayed, max_displayed_len as i32);
    }
}

#[derive(Clone, Debug)]
pub struct PromptState {
    pub name: String,
    pub visible: bool
}

impl PromptState {
    pub fn new(name: impl Into<String>, visible: bool) -> Self {
        let name = name.into();
        Self { name, visible }
    }
}

impl Default for PromptState {
    fn default() -> Self {
        Self::new("", false)
    }
}