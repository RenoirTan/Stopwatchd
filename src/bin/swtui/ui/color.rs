#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ColorPair {
    /// Active items, windows and borders
    #[default]
    Active = 1,
    /// Inactive items, windows and borders
    Inactive = 2,
    /// An item being hovered over
    Selected = 3
}

impl ColorPair {
    pub fn set_color(&self, window: &pancurses::Window, bold: bool) {
        let mut color = pancurses::COLOR_PAIR(*self as i16 as pancurses::chtype);
        if color == 0 {
            panic!("PLEASE RUN swtui::ui::color::init_color!");
        }
        if bold {
            color |= pancurses::A_BOLD;
        }
        window.attrset(color);
    }
}

pub fn init_color() {
    if !pancurses::has_colors() {
        return;
    }

    pancurses::start_color();
    let bg = if pancurses::use_default_colors() == pancurses::OK {
        -1
    } else {
        pancurses::COLOR_BLACK
    };

    pancurses::init_pair(ColorPair::Active as i16, pancurses::COLOR_WHITE, bg);
    pancurses::init_pair(ColorPair::Inactive as i16, pancurses::COLOR_BLACK, bg);
    pancurses::init_pair(ColorPair::Selected as i16, pancurses::COLOR_CYAN, bg);
}
