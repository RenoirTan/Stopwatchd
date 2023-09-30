use stopwatchd::{communication::details::StopwatchDetails, identifiers::Identifier};

use crate::{ui::{Ui, color::ColorPair}, util::center_text};

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
        let FocusPanelState { selected: _selected, details } = state;
        self.clear(ui);
        let (left, right, top, bottom) = ui.borders_geometry().focus_panel_geometry();
        if let Some(ref d) = details {
            let display_name = d.identifier.to_string();
            let (l_x, r_x) = center_text(display_name.len(), (left, right)).unwrap();
            ui.window.mvaddnstr(top, l_x as i32, &display_name, (r_x - l_x + 1) as i32);
            let row_filler = "x".repeat((right - left + 1) as usize);
            for y in top+1..=bottom {
                ui.window.mvaddnstr(y, left, &row_filler, row_filler.len() as i32);
            }
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
