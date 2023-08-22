use stopwatchd::{
    identifiers::{Identifier, Name},
    models::stopwatch::Stopwatch
};
use crate::{
    ui::{Ui, color::ColorPair},
    util::center_text
};

pub struct ListPanel;

impl ListPanel {
    pub fn clear(&self, ui: &Ui) {
        let (left, right, top, bottom) = ui.borders_geometry().list_panel_geometry();
        ColorPair::Active.set_color(&ui.window, false);
        for x in left..=right {
            for y in top..=bottom {
                ui.window.mvaddch(y, x, ' ');
            }
        }
    }

    pub fn draw(&self, ui: &Ui, state: &ListPanelState) {
        let ListPanelState { identifiers: stopwatches, selected, start } = state;
        self.clear(ui); // reset the screen
        // nothing to do if no stopwatches
        if stopwatches.len() == 0 {
            return;
        }
        let (left, right, top, bottom) = ui.borders_geometry().list_panel_geometry();
        // number of stopwatches that can fit on screen
        let height = (bottom - top + 1) as usize;
        for i in 0..height {
            let index = start + i;
            if index >= stopwatches.len() { break; } // goodbye
            if index == *selected {
                ColorPair::Selected.set_color(&ui.window, true);
            } else {
                ColorPair::Inactive.set_color(&ui.window, false);
            }
            let y = top + i as i32; // where to write
            let identifier = stopwatches[index].to_string();
            let (l_x, r_x) = center_text(identifier.len(), (left as usize, right as usize))
                .unwrap();
            let l_x = l_x as i32;
            let r_x = r_x as i32;
            // only write the first r_x - l_x + 1 characters of the identifier
            ui.window.mvaddnstr(y, l_x, identifier, r_x - l_x + 1);
        }
        assert!(*selected < stopwatches.len());
    }
}

#[derive(Clone, Debug)]
pub struct ListPanelState {
    pub identifiers: Vec<Identifier>,
    pub selected: usize,
    pub start: usize
}

impl ListPanelState {
    pub fn generate_fake_names(number_of_stopwatches: usize) -> Self {
        let mut identifiers = Vec::new();
        for i in 0..number_of_stopwatches {
            let i_s = format!("{}", i);
            let name = Name::fixed(match i % 5 {
                0 => &i_s,
                1 => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890",
                2 => "super duper duper duper duper duper duper duper duper long name",
                3 => "bruh",
                4 => "",
                _ => panic!("impossible")
            });
            let stopwatch = Stopwatch::new(name);
            identifiers.push(stopwatch.identifier);
        }
        let selected = 0;
        let start = 0;
        Self { identifiers, selected, start }
    }
}

impl Default for ListPanelState {
    fn default() -> Self {
        let identifiers = vec![];
        let selected = 0;
        let start = 0;
        Self { identifiers, selected, start }
    }
}