use crate::ui::{
    color::ColorPair,
    Ui
};

pub struct Bar;

impl Bar {
    pub fn draw(&self, ui: &Ui, focus_active: bool) {
        self.draw_background(ui);
        if focus_active {
            self.draw_focus_panel_shortcuts(ui);
        } else {
            self.draw_list_panel_shortcuts(ui);
        }
    }

    fn draw_background(&self, ui: &Ui) {
        let bar_location = ui.bar_location();
        let width = ui.dimensions().x;
        ColorPair::Bar.set_color(&ui.window, false);
        let bar = " ".repeat(width as usize);
        ui.window.mvaddstr(bar_location.y, 0, bar);
    }

    fn draw_list_panel_shortcuts(&self, ui: &Ui) {
        let y = ui.bar_location().y;
        // let width = ui.dimensions().x;
        
        // Up
        ColorPair::BarKey.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 0, "Up");
        ColorPair::Bar.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 2, "Scroll Up");

        // Down
        ColorPair::BarKey.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 12, "Down");
        ColorPair::Bar.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 16, "Scroll Down");

        self.draw_global_shortcuts(ui, 28);
    }

    fn draw_focus_panel_shortcuts(&self, ui: &Ui) {
        let y = ui.bar_location().y;

        // Space: Play or Pause
        // TODO: Pass a state as an argument that tells the bar whether
        // the current stopwatch is playing or not
        ColorPair::BarKey.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 0, "Space");
        ColorPair::Bar.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 5, "Play");

        // Enter: Lap
        ColorPair::BarKey.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 10, "Enter");
        ColorPair::Bar.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 15, "New Lap");

        // S: Stop
        ColorPair::BarKey.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 23, "S");
        ColorPair::Bar.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 24, "Stop");

        // D: Delete
        ColorPair::BarKey.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 29, "D");
        ColorPair::Bar.set_color(&ui.window, false);
        ui.window.mvaddstr(y, 30, "Delete");

        self.draw_global_shortcuts(ui, 37);
    }

    fn draw_global_shortcuts(&self, ui: &Ui, min_x: i32) {
        let y = ui.bar_location().y;

        // F10: Quit
        ColorPair::BarKey.set_color(&ui.window, false);
        ui.window.mvaddstr(y, min_x+0, "F10");
        ColorPair::Bar.set_color(&ui.window, false);
        ui.window.mvaddstr(y, min_x+3, "Quit");
    }
}
