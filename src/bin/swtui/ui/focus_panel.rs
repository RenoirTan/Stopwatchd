use stopwatchd::{communication::details::StopwatchDetails, identifiers::Identifier};

use crate::ui::{Ui, color::ColorPair};

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
        let (left, right, top, bottom) = ui.borders_geometry().focus_panel_geometry();
        if let Some(ref _d) = details {
            for x in left..=right {
                for y in top..=bottom {
                    ui.window.mvaddch(y, x, 'x');
                }
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
