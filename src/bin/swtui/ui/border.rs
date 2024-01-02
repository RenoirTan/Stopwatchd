use crate::ui::{color::ColorPair, geometry::Size};

pub struct Border {
    pub top_left_char: char,
    pub top_right_char: char,
    pub bottom_left_char: char,
    pub bottom_right_char: char,
    pub horizontal_char: char,
    pub vertical_char: char
}

impl Border {
    pub fn new_unicode() -> Self {
        Self {
            top_left_char: '┌',
            top_right_char: '┐',
            bottom_left_char: '└',
            bottom_right_char: '┘',
            horizontal_char: '─',
            vertical_char: '│'
        }
    }

    pub fn new_ascii() -> Self {
        Self {
            top_left_char: '+',
            top_right_char: '+',
            bottom_left_char: '+',
            bottom_right_char: '+',
            horizontal_char: '-',
            vertical_char: '|'
        }
    }

    // focus_active: Whether the focus (right) panel is active
    /* pub fn draw(&self, ui: &Ui) {
        let (list_panel_color, focus_panel_color) = if ui.focus_active {
            (ColorPair::Inactive, ColorPair::Selected)
        } else {
            (ColorPair::Selected, ColorPair::Inactive)
        };

        self.border(&ui.list_panel.window, list_panel_color, false);
        self.border(&ui.focus_panel.window, focus_panel_color, false);
    } */

    pub fn border(&self, window: &pancurses::Window, color: ColorPair, bold: bool) {
        color.set_color(window, bold);
        // window.draw_box(self.vertical_char, self.horizontal_char);
        /* window.border(
            self.vertical_char,
            self.vertical_char,
            self.horizontal_char,
            self.horizontal_char,
            self.top_left_char,
            self.top_right_char,
            self.bottom_left_char,
            self.bottom_right_char
        ); */
        self.inner_border(window);
    }

    fn inner_border(&self, window: &pancurses::Window) {
        let size = Size::window_dimensions(window);
        window.mvaddstr(0, 0, self.top_left_char.to_string());
        window.mvaddstr(0, size.x-1, self.top_right_char.to_string());
        window.mvaddstr(size.y-1, 0, self.bottom_left_char.to_string());
        window.mvaddstr(size.y-1, size.x-1, self.bottom_right_char.to_string());
        for x in 1..size.x-1 {
            window.mvaddstr(0, x, self.horizontal_char.to_string());
            window.mvaddstr(size.y-1, x, self.horizontal_char.to_string());
        }
        for y in 1..size.y-1 {
            window.mvaddstr(y, 0, self.vertical_char.to_string());
            window.mvaddstr(y, size.x-1, self.vertical_char.to_string());
        }
    }
}
