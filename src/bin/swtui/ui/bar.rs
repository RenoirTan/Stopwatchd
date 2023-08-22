use crate::ui::{
    color::ColorPair,
    Ui
};

pub struct Bar;

impl Bar {
    pub fn draw(&self, ui: &Ui, focus_active: bool) {
        let mut x = 0;
        self.draw_background(ui);
        if focus_active {
            self.draw_focus_panel_shortcuts(ui, &mut x);
        } else {
            self.draw_list_panel_shortcuts(ui, &mut x);
        }
    }

    fn draw_background(&self, ui: &Ui) {
        let bar_location = ui.bar_location();
        let width = ui.dimensions().x;
        ColorPair::Bar.set_color(&ui.window, false);
        let bar = " ".repeat(width as usize);
        ui.window.mvaddstr(bar_location.y, 0, bar);
    }

    fn draw_list_panel_shortcuts(&self, ui: &Ui, x: &mut i32) {
        let y = ui.bar_location().y;
        // let width = ui.dimensions().x;

        ColorPair::BarKey.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Right");
        ColorPair::Bar.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Select ");
        
        // Up
        ColorPair::BarKey.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Up");
        ColorPair::Bar.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Scroll Up ");

        // Down
        ColorPair::BarKey.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Down");
        ColorPair::Bar.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Scroll Down ");

        self.draw_global_shortcuts(ui, x);
    }

    fn draw_focus_panel_shortcuts(&self, ui: &Ui, x: &mut i32) {
        let y = ui.bar_location().y;

        ColorPair::BarKey.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Left");
        ColorPair::Bar.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Back ");

        // Space: Play or Pause
        // TODO: Pass a state as an argument that tells the bar whether
        // the current stopwatch is playing or not
        ColorPair::BarKey.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Space");
        ColorPair::Bar.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Play ");

        // Enter: Lap
        ColorPair::BarKey.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Enter");
        ColorPair::Bar.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "New Lap ");

        // S: Stop
        ColorPair::BarKey.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "S");
        ColorPair::Bar.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Stop ");

        // D: Delete
        ColorPair::BarKey.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "D");
        ColorPair::Bar.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Delete ");

        self.draw_global_shortcuts(ui, x);
    }

    fn draw_global_shortcuts(&self, ui: &Ui, x: &mut i32) {
        let y = ui.bar_location().y;

        // F10: Quit
        ColorPair::BarKey.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "F10");
        ColorPair::Bar.set_color(&ui.window, false);
        *x = ui.add_string(*x, y, "Quit ");
    }
}
