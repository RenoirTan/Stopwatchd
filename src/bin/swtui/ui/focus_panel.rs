use std::sync::Arc;

use stopwatchd::{
    communication::details::StopwatchDetails,
    identifiers::Identifier
};

use crate::{
    ui::{Ui, color::ColorPair, geometry::BordersGeometry},
    util::center_text
};

pub struct FocusPanel {
    pub window: Arc<pancurses::Window>
}

impl FocusPanel {
    pub fn newwin(g: BordersGeometry) -> pancurses::Window {
        let nlines = g.bottom_right.y - g.top_left.y; // lowest row ignored for bar
        let ncols = g.bottom_right.x - g.focus_x + 1;
        let begy = g.top_left.y;
        let begx = g.focus_x;
        pancurses::newwin(nlines, ncols, begy, begx)
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
        ui.border.border(
            &self.window,
            if ui.is_focus_active() { ColorPair::Selected } else { ColorPair::Inactive },
            false
        );
    }

    pub fn draw(&self, ui: &Ui) {
        let FocusPanelState { selected, details } = &ui.focus_panel_state;
        self.clear();
        self.border(ui);
        let (left, right, top, bottom) = self.geometry();
        if let Some(ref d) = details {
            // Name
            let display_name = d.identifier.to_string();
            let (l_x, r_x) = center_text(display_name.len(), (left, right)).unwrap();
            ColorPair::Active.set_color(&self.window, true);
            self.window.mvaddnstr(top, l_x, &display_name, r_x - l_x + 1);

            // Time
            ColorPair::Active.set_color(&self.window, false);
            let display_time = ui.formatter.format_duration(d.total_time);
            let (l_x, r_x) = center_text(display_time.len(), (left, right)).unwrap();
            self.window.mvaddnstr(top+1, l_x, &display_time, r_x - l_x + 1);

            // State
            ColorPair::Active.set_color(&self.window, true);
            let display_state = format!("{}", d.state);
            let (l_x, r_x) = center_text(display_state.len(), (left, right)).unwrap();
            self.window.mvaddnstr(top+2, l_x, &display_state, r_x - l_x + 1);
        } else if let Some(identifier) = selected.as_ref() {
            ColorPair::Active.set_color(&self.window, false);
            let err_msg = "Could not find:";
            let mid_y = (bottom - top) / 2;
            let (l_x, r_x) = center_text(err_msg.len(), (left, right)).unwrap();
            self.window.mvaddnstr(mid_y, l_x, err_msg, r_x - l_x + 1);
            let id_str = identifier.to_string();
            let (l_x, r_x) = center_text(id_str.len(), (left, right)).unwrap();
            self.window.mvaddnstr(mid_y + 1, l_x, id_str, r_x - l_x + 1);
        } else {
            ColorPair::Active.set_color(&self.window, true);
            let welcome = "Stopwatchd";
            let mid_y = (bottom - top) / 2;
            let (l_x, r_x) = center_text(welcome.len(), (left, right)).unwrap();
            self.window.mvaddnstr(mid_y, l_x, welcome, r_x - l_x + 1);
        }
    }

    pub fn refresh(&self) {
        self.window.refresh();
    }
}

pub struct FocusPanelState {
    pub selected: Option<Identifier>,
    pub details: Option<StopwatchDetails>
}

impl FocusPanelState {
    pub fn new(selected: Option<Identifier>, details: Option<StopwatchDetails>) -> Self {
        Self { selected, details }
    }
}

impl Default for FocusPanelState {
    fn default() -> Self {
        Self::new(None, None)
    }
}
