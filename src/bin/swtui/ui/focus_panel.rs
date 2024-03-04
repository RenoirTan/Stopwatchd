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
    pub fn newwin(main: &pancurses::Window, g: BordersGeometry) -> pancurses::Window {
        let nlines = g.bottom_right.y - g.top_left.y; // lowest row ignored for bar
        let ncols = g.bottom_right.x - g.focus_x + 1;
        let begy = g.top_left.y;
        let begx = g.focus_x;
        main.subwin(nlines, ncols, begy, begx).unwrap()
    }

    pub fn new(window: Arc<pancurses::Window>) -> Self {
        Self { window }
    }

    pub fn geometry(&self) -> (i32, i32, i32, i32) {
        let (max_y, max_x) = self.window.get_max_yx();
        (1, max_x-2, 1, max_y-2) // don't include border
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
        let FocusPanelState { selected, details, .. } = &ui.focus_panel_state;
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
            let display_time = format!("Total: {}", ui.formatter.format_duration(d.total_time));
            let (l_x, r_x) = center_text(display_time.len(), (left, right)).unwrap();
            self.window.mvaddnstr(top+1, l_x, &display_time, r_x - l_x + 1);

            // State
            ColorPair::Active.set_color(&self.window, true);
            let display_state = format!("{}", d.state);
            let (l_x, r_x) = center_text(display_state.len(), (left, right)).unwrap();
            self.window.mvaddnstr(top+2, l_x, &display_state, r_x - l_x + 1);

            // Lap Time
            ColorPair::Active.set_color(&self.window, false);
            let lap_time = format!("Lap: {}", ui.formatter.format_duration(d.current_lap_time()));
            let (l_x, r_x) = center_text(lap_time.len(), (left, right)).unwrap();
            self.window.mvaddnstr(top+3, l_x, &lap_time, r_x - l_x + 1);

            // Lap Count
            ColorPair::Active.set_color(&self.window, false);
            let lap_count = format!("Lap Count: {}", d.laps_count());
            let (l_x, r_x) = center_text(lap_count.len(), (left, right)).unwrap();
            self.window.mvaddnstr(top+4, l_x, &lap_count, r_x - l_x + 1);

            // Display all laps if exists
            if let Some(ref vi) = d.verbose_info {
                ColorPair::Active.set_color(&self.window, true);
                let display_laps = "Laps:";
                let (l_x, r_x) = center_text(display_laps.len(), (left, right)).unwrap();
                self.window.mvaddnstr(top+5, l_x, display_laps, r_x - l_x + 1);

                ColorPair::Active.set_color(&self.window, false);
                let mut row = top + 6;
                let lap_scroll = ui.focus_panel_state.lap_scroll;
                // latest laps first
                for (index, lap) in vi.laps.iter().rev().skip(lap_scroll).enumerate() {
                    if row > bottom {
                        break;
                    }

                    let lap_number = d.laps_count() - index - lap_scroll;
                    let lap_time = ui.formatter.format_duration(lap.duration);
                    let display_lap = format!("{}: {}", lap_number, lap_time);
                    let (l_x, r_x) = center_text(display_lap.len(), (left, right)).unwrap();
                    self.window.mvaddnstr(row, l_x, &display_lap, r_x - l_x + 1);

                    row += 1;
                }
            }
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
}

pub struct FocusPanelState {
    pub selected: Option<Identifier>,
    pub details: Option<StopwatchDetails>,
    pub lap_scroll: usize,
}

impl FocusPanelState {
    pub fn new(
        selected: Option<Identifier>,
        details: Option<StopwatchDetails>,
        lap_scroll: usize
    ) -> Self {
        Self { selected, details, lap_scroll }
    }

    pub fn update_selected(&mut self, identifier: Option<Identifier>) {
        if self.selected != identifier {
            self.lap_scroll = 0;
        }
        self.selected = identifier;
    }

    pub fn update(&mut self, details: Option<StopwatchDetails>) {
        match details {
            Some(d) => {
                self.update_selected(Some(d.identifier.clone()));
                self.details = Some(d);
            },
            None => {
                self.selected = None;
                self.details = None;
                self.lap_scroll = 0;
            }
        }
    }

    pub fn scroll_inner(&mut self, up: bool) {
        if up {
            // only scroll up if within bound
            if self.lap_scroll >= 1 {
                self.lap_scroll -= 1;
            }
        } else {
            // only scroll down if definitely within bound
            if let Some(ref d) = self.details {
                if self.lap_scroll < d.laps_count() {
                    self.lap_scroll += 1;
                }
            }
        }
    }
}

impl Default for FocusPanelState {
    fn default() -> Self {
        Self::new(None, None, 0)
    }
}
