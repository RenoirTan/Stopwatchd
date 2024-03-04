use std::sync::Arc;

use stopwatchd::identifiers::{Identifier, Name, UniqueId};
use crate::{
    ui::{Ui, color::ColorPair, geometry::BordersGeometry},
    util::center_text
};

pub struct ListPanel {
    pub window: Arc<pancurses::Window>
}

impl ListPanel {
    pub fn newwin(main: &pancurses::Window, g: BordersGeometry) -> pancurses::Window {
        let nlines = g.bottom_right.y - g.top_left.y; // lowest row ignored for bar
        let ncols = g.focus_x - g.top_left.x;
        let begy = g.top_left.y;
        let begx = g.top_left.x;
        main.subwin(nlines, ncols, begy, begx).unwrap()
    }

    pub fn new(window: Arc<pancurses::Window>) -> Self {
        Self { window }
    }

    pub fn geometry(&self) -> (i32, i32, i32, i32) {
        let (max_y, max_x) = self.window.get_max_yx();
        (1, max_x-2, 1, max_y-2)
    }

    pub fn height(&self) -> i32 {
        let (_l, _r, top, bottom) = self.geometry();
        bottom - top + 1
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
            if ui.is_focus_active() { ColorPair::Inactive } else { ColorPair::Selected },
            false
        );
    }

    pub fn draw(&self, ui: &Ui) {
        let ListPanelState { identifiers: stopwatches, selected, start } = &ui.list_panel_state;
        self.clear(); // reset the screen
        self.border(ui);
        // nothing to do if no stopwatches
        if stopwatches.len() == 0 {
            return;
        }
        let (left, right, top, bottom) = self.geometry();
        // number of stopwatches that can fit on screen
        let height = (bottom - top + 1) as usize;
        for i in 0..height {
            let index = start + i;
            if index >= stopwatches.len() { break; } // goodbye
            if index == *selected {
                ColorPair::Selected.set_color(&self.window, true);
            } else {
                ColorPair::Inactive.set_color(&self.window, false);
            }
            let y = top + i as i32; // where to write
            let identifier = stopwatches[index].to_string();
            let (l_x, r_x) = center_text(identifier.len(), (left, right)).unwrap();
            let l_x = l_x as i32;
            let r_x = r_x as i32;
            // only write the first r_x - l_x + 1 characters of the identifier
            self.window.mvaddnstr(y, l_x, identifier, r_x - l_x + 1);
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
    #[allow(unused)]
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
            let identifier = Identifier::new(UniqueId::generate(), name);
            identifiers.push(identifier);
        }
        let selected = 0;
        let start = 0;
        Self { identifiers, selected, start }
    }

    pub (in crate::ui) fn scroll_inner(&mut self, up: bool, window_height: usize) {
        if up {
            self.selected = if self.selected <= 0 { 0 } else { self.selected - 1 };
        } else {
            self.selected = if self.identifiers.len() == 0 {
                0
            } else if self.selected + 1 >= self.identifiers.len() {
                self.identifiers.len() - 1
            } else {
                self.selected + 1
            };
        }
        self.selected_in_range(up, window_height);
    }

    pub (in crate::ui) fn scroll_home(&mut self, window_height: usize) {
        self.selected = 0;
        self.selected_in_range(true, window_height);
    }

    pub (in crate::ui) fn scroll_end(&mut self, window_height: usize) {
        self.selected = if self.identifiers.len() == 0 {
            0
        } else {
            self.identifiers.len() - 1
        };
        self.selected_in_range(false, window_height);
    }

    fn selected_in_range(&mut self, up: bool, window_height: usize) {
        let up_index = self.start;
        let down_index = self.start + window_height;
        if up && (self.selected < up_index || self.selected >= down_index) {
            self.start = self.selected;
        } else if !up && (self.selected < up_index || self.selected >= down_index) {
            self.start = if
                self.identifiers.len() < window_height || self.selected < window_height
            {
                0
            } else {
                self.selected - window_height + 1
            };
        }
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
