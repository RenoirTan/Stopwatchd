use stopwatchd::{
    communication::details::StopwatchDetails,
    identifiers::Identifier
};

use crate::{
    ui::{Ui, color::ColorPair},
    util::center_text
};

pub struct FocusPanel;

impl FocusPanel {
    pub fn clear(&self, ui: &Ui) {
        let (left, right, top, bottom) = ui.borders_geometry().focus_panel_geometry();
        ColorPair::Active.set_color(&ui.window, false);
        for x in left..=right {
            for y in top..=bottom {
                ui.window.mvaddch(y, x, ' ');
            }
        }
    }

    pub fn draw(&self, ui: &Ui, state: &FocusPanelState) {
        let FocusPanelState { selected, details } = state;
        self.clear(ui);
        let (left, right, top, bottom) = ui.borders_geometry().focus_panel_geometry();
        if let Some(ref d) = details {
            // Name
            let display_name = d.identifier.to_string();
            let (l_x, r_x) = center_text(display_name.len(), (left, right)).unwrap();
            ColorPair::Active.set_color(&ui.window, true);
            ui.window.mvaddnstr(top, l_x, &display_name, r_x - l_x + 1);

            // Time
            ColorPair::Active.set_color(&ui.window, false);
            let display_time = ui.formatter.format_duration(d.total_time);
            let (l_x, r_x) = center_text(display_time.len(), (left, right)).unwrap();
            ui.window.mvaddnstr(top+1, l_x, &display_time, r_x - l_x + 1);

            // State
            ColorPair::Active.set_color(&ui.window, true);
            let display_state = format!("{}", d.state);
            let (l_x, r_x) = center_text(display_state.len(), (left, right)).unwrap();
            ui.window.mvaddnstr(top+2, l_x, &display_state, r_x - l_x + 1);
        } else if let Some(identifier) = selected.as_ref() {
            ColorPair::Active.set_color(&ui.window, false);
            let err_msg = "Could not find:";
            let mid_y = (bottom - top) / 2;
            let (l_x, r_x) = center_text(err_msg.len(), (left, right)).unwrap();
            ui.window.mvaddnstr(mid_y, l_x, err_msg, r_x - l_x + 1);
            let id_str = identifier.to_string();
            let (l_x, r_x) = center_text(id_str.len(), (left, right)).unwrap();
            ui.window.mvaddnstr(mid_y + 1, l_x, id_str, r_x - l_x + 1);
        } else {
            ColorPair::Active.set_color(&ui.window, true);
            let welcome = "Stopwatchd";
            let mid_y = (bottom - top) / 2;
            let (l_x, r_x) = center_text(welcome.len(), (left, right)).unwrap();
            ui.window.mvaddnstr(mid_y, l_x, welcome, r_x - l_x + 1);
        }
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
