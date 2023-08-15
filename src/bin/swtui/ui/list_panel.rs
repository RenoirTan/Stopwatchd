use stopwatchd::communication::details::StopwatchDetails;
use crate::ui::{Ui, color::ColorPair};

pub struct ListPanel;

impl ListPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn clear(&self, ui: &Ui) {
        let (left, right, top, bottom) = ui.borders_geometry().list_panel_geometry();
        ColorPair::Active.set_color(&ui.window, false);
        for x in left..=right {
            for y in top..=bottom {
                ui.window.mvaddch(y, x, 'a');
            }
        }
    }

    pub fn draw(&self, ui: &Ui, stopwatches: &[StopwatchDetails], selected: usize) {
        if stopwatches.len() == 0 {
            return;
        }
        assert!(selected < stopwatches.len());
    }
}
