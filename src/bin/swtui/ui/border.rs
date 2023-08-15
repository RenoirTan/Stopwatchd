use crate::ui::{Ui, color::ColorPair};

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
    pub fn draw(&self, ui: &Ui, focus_active: bool) {
        let g = ui.borders_geometry();
        let (list_panel_color, focus_panel_color) = if focus_active {
            (ColorPair::Inactive, ColorPair::Selected)
        } else {
            (ColorPair::Selected, ColorPair::Inactive)
        };

        // we're working on the list panel (left) now
        list_panel_color.set_color(&ui.window, false);

        // draw top left and bottom left characters
        ui.window.mvaddstr(g.top_left.y, g.top_left.x, self.top_left_char.to_string());
        ui.window.mvaddstr(g.bottom_right.y, g.top_left.x, self.bottom_left_char.to_string());
        for x in 1..g.separator_x {
            // draw top border
            ui.window.mvaddstr(g.top_left.y, x, self.horizontal_char.to_string());
            // draw bottom border
            ui.window.mvaddstr(g.bottom_right.y, x, self.horizontal_char.to_string());
        }
        for y in 1..g.bottom_right.y {
            // draw left border
            ui.window.mvaddstr(y, g.top_left.x, self.vertical_char.to_string());
        } 

        // now we're working on the focus panel (right)
        focus_panel_color.set_color(&ui.window, false);

        // draw top right and bottom right characters
        ui.window.mvaddstr(g.top_left.y, g.bottom_right.x, self.top_right_char.to_string());
        ui.window.mvaddstr(g.bottom_right.y, g.bottom_right.x, self.bottom_right_char.to_string());
        for x in g.separator_x+1..g.bottom_right.x {
            // top border
            ui.window.mvaddstr(g.top_left.y, x, self.horizontal_char.to_string());
            // bottom border
            ui.window.mvaddstr(g.bottom_right.y, x, self.horizontal_char.to_string());
        }
        for y in 1..g.bottom_right.y {
            // right border
            ui.window.mvaddstr(y, g.bottom_right.x, self.vertical_char.to_string());
        }

        // draw central separator
        ColorPair::Selected.set_color(&ui.window, false);
        ui.window.mvaddstr(g.top_left.y, g.separator_x, if focus_active {
            self.top_left_char.to_string()
        } else {
            self.top_right_char.to_string()
        });
        ui.window.mvaddstr(g.bottom_right.y, g.separator_x, if focus_active {
            self.bottom_left_char.to_string()
        } else {
            self.bottom_right_char.to_string()
        });
        for y in 1..g.bottom_right.y {
            ui.window.mvaddstr(y, g.separator_x, self.vertical_char.to_string());
        }
    }
}
