pub mod bar;
pub mod border;
pub mod color;
pub mod focus_panel;
pub mod geometry;
pub mod list_panel;
pub mod prompt;

use std::{
    sync::Arc,
    path::PathBuf
};

use stopwatchd::{
    communication::{
        client::{ClientSender, Request},
        reply_specifics::{InfoAnswer, SpecificAnswer},
        request_specifics::StartArgs,
    },
    fmt::Formatter,
    models::stopwatch::State
};

use self::{
    bar::Bar,
    border::Border,
    geometry::{Size, BordersGeometry, BarLocation},
    list_panel::{ListPanel, ListPanelState},
    focus_panel::{FocusPanel, FocusPanelState},
    prompt::{Prompt, PromptState}
};

pub struct Ui {
    pub window: Arc<pancurses::Window>,
    pub border: Border,
    pub list_panel: ListPanel,
    pub list_panel_state: ListPanelState,
    pub focus_panel: FocusPanel,
    pub focus_panel_state: FocusPanelState,
    pub prompt: Prompt,
    pub prompt_state: PromptState,
    pub bar: Bar,
    focus_active: bool,
    pub formatter: Formatter,
    pub ssock_path: PathBuf
}

impl Ui {
    pub fn new(
        window: Arc<pancurses::Window>,
        border: Border,
        list_panel_state: ListPanelState,
        focus_panel_state: FocusPanelState,
        prompt_state: PromptState,
        bar: Bar,
        focus_active: bool,
        formatter: Formatter,
        ssock_path: PathBuf
    ) -> Self {
        window.nodelay(false);
        window.keypad(true);
        pancurses::noecho();
        let g = BordersGeometry::from_window(&window);
        let list_panel = ListPanel::new(Arc::new(ListPanel::newwin(&window, g)));
        let focus_panel = FocusPanel::new(Arc::new(FocusPanel::newwin(&window, g)));
        let prompt = Prompt::new(Arc::new(Prompt::newwin(&window)));
        Self {
            window,
            border,
            list_panel,
            list_panel_state,
            focus_panel,
            focus_panel_state,
            prompt,
            prompt_state,
            bar,
            focus_active,
            formatter,
            ssock_path
        }
    }

    pub async fn refresh_list(&mut self) {
        let request = Request::info_all(false);
        
        let reply = ClientSender::new(&self.ssock_path).send(request).await.unwrap();

        if !reply.errors.is_empty() {
            panic!("reply to refresh request returns error, requires reworking");
        }
        if let SpecificAnswer::Info(InfoAnswer::All(ref all)) = reply.specific_answer {
            self.list_panel_state.identifiers = all.access_order
                .iter()
                .map(|s| reply.successful.get(s).unwrap().identifier.clone())
                .collect();
        } else {
            panic!("refresh request should be replied with InfoAnswer::All");
        }
    }

    // TODO: when refreshing a stopwatch, the access order changes and mucks
    // everything up
    pub async fn refresh_stopwatch(&mut self) {
        if self.focus_panel_state.selected.is_none() { return; }
        let identifier = self.focus_panel_state.selected.as_ref().unwrap();
        let raw_id = identifier.to_string();
        let request = Request::info_some(vec![raw_id.clone()], true);
        
        let mut reply = ClientSender::new(&self.ssock_path).send(request).await.unwrap();

        if !reply.errors.is_empty() {
            panic!("reply to stopwatch refresh request returns error, requires reworking");
        }
        if let SpecificAnswer::Info(InfoAnswer::Basic) = reply.specific_answer {
            self.focus_panel_state.update(reply.successful.remove(&raw_id));
        } else {
            panic!("refresh request should be replied with InfoAnswer::All");
        }
    }

    pub fn draw(&self) {
        self.bar.draw(self, self.focus_active);
        // self.window.refresh(); // must be placed here or the other windows will be cleared
        self.list_panel.draw(self);
        // self.list_panel.refresh();
        self.focus_panel.draw(self);
        // self.focus_panel.refresh();
        if self.prompt_state.visible {
            self.prompt.draw(self);
            // self.prompt.refresh();
        }
        self.window.touch();
        self.window.refresh();
    }

    /// (rows, columns) or (y, x)
    pub fn dimensions(&self) -> Size {
        Size::window_dimensions(&self.window)
    }

    pub fn borders_geometry(&self) -> BordersGeometry {
        BordersGeometry::from_window(&self.window)
    }

    pub fn bar_location(&self) -> BarLocation {
        let Size { x: _x, y } = self.dimensions();
        BarLocation { y: y-1 }
    }

    pub fn add_string<S: AsRef<str>>(&self, x: i32, y: i32, s: S) -> i32 {
        self.window.mvaddstr(y, x, &s);
        x + s.as_ref().len() as i32
    }

    pub fn scroll_list_panel(&mut self, up: bool) {
        let height = self.list_panel.height();
        self.list_panel_state.scroll_inner(up, height as usize);
    }

    pub fn scroll_list_home(&mut self) {
        let height = self.list_panel.height();
        self.list_panel_state.scroll_home(height as usize);
    }

    pub fn scroll_list_end(&mut self) {
        let height = self.list_panel.height();
        self.list_panel_state.scroll_end(height as usize);
    }

    pub fn scroll_focus_panel(&mut self, up: bool) {
        self.focus_panel_state.scroll_inner(up);
    }
    pub async fn set_focus_active(&mut self, yes: bool) {
        self.focus_active = yes;
        if yes {
            let new_identifier = self.list_panel_state.identifiers
                .get(self.list_panel_state.selected)
                .cloned();
            self.focus_panel_state.update_selected(new_identifier);
            match self.focus_panel_state.selected {
                Some(ref _id) => self.refresh_stopwatch().await,
                None => self.focus_panel_state.update(None)
            }
        }
    }

    pub fn is_focus_active(&self) -> bool {
        self.focus_active
    }

    pub async fn toggle_state(&mut self) {
        let (mut reply, identifier) = if let Some(ref mut d) = self.focus_panel_state.details {
            let request = match d.state {
                State::Playing => Request::pause(vec![d.identifier.to_string()], true),
                State::Paused => Request::play(vec![d.identifier.to_string()], true),
                State::Ended => return ()
            };

            let reply = ClientSender::new(&self.ssock_path).send(request).await.unwrap();

            if reply.errors.len() >= 1 {
                error!("[swtui::ui::Ui::toggle_state] uh oh");
            }

            (reply, d.identifier.clone())
        } else {
            return ();
        };

        if matches!(reply.specific_answer, SpecificAnswer::Play(_) | SpecificAnswer::Pause(_)) {
            self.focus_panel_state.update(reply.successful.remove(&identifier.to_string()));
        } else {
            panic!("server did not reply with SpecificAnswer::Play or Pause!");
        }
    }

    pub fn prompt_name(&mut self) {
        self.prompt_state.visible = true;
    }

    pub async fn start_stopwatch(&mut self) {
        let name = self.prompt_state.name.clone();
        let request = Request::start(vec![name], true, StartArgs { fix_bad_names: true });
        
        let reply = ClientSender::new(&self.ssock_path).send(request).await.unwrap();

        if reply.errors.len() >= 1 {
            error!("[swtui::ui::Ui::start_stopwatch] uh oh");
        }

        self.refresh_list().await;
    }

    pub async fn stop_stopwatch(&mut self) {
        let (mut reply, identifier) = if let Some(ref mut d) = self.focus_panel_state.details {
            let request = match d.state {
                State::Playing | State::Paused => Request::stop(
                    vec![d.identifier.to_string()],
                    true
                ),
                State::Ended => return ()
            };

            let reply = ClientSender::new(&self.ssock_path).send(request).await.unwrap();

            if reply.errors.len() >= 1 {
                error!("[swtui::ui::Ui::stop_stopwatch] uh oh");
            }

            (reply, d.identifier.clone())
        } else {
            return ();
        };

        if let SpecificAnswer::Stop(_) = reply.specific_answer {
            self.focus_panel_state.update(reply.successful.remove(&identifier.to_string()));
        } else {
            panic!("server did not reply with SpecificAnswer::Stop!");
        }
    }

    pub async fn lap_stopwatch(&mut self) {
        let (mut reply, identifier) = if let Some(ref mut d) = self.focus_panel_state.details {
            let request = match d.state {
                State::Playing | State::Paused => Request::lap(
                    vec![d.identifier.to_string()],
                    true
                ),
                State::Ended => return ()
            };

            let reply = ClientSender::new(&self.ssock_path).send(request).await.unwrap();

            if reply.errors.len() >= 1 {
                error!("[swtui::ui::Ui::lap_stopwatch] uh oh");
            }

            (reply, d.identifier.clone())
        } else {
            return ();
        };

        if let SpecificAnswer::Lap(_) = reply.specific_answer {
            self.focus_panel_state.update(reply.successful.remove(&identifier.to_string()));
        } else {
            panic!("server did not reply with SpecificAnswer::Lap!");
        }
    }

    pub async fn delete_stopwatch(&mut self) {
        let reply = if let Some(ref mut d) = self.focus_panel_state.details {
            let request = Request::delete(vec![d.identifier.to_string()], true);

            let reply = ClientSender::new(&self.ssock_path).send(request).await.unwrap();

            if reply.errors.len() >= 1 {
                error!("[swtui::ui::Ui::delete_stopwatch] uh oh");
            }

            reply
        } else {
            return ();
        };

        if let SpecificAnswer::Delete(_) = reply.specific_answer {
            self.focus_panel_state.update(None);
        } else {
            panic!("server did not reply with SpecificAnswer::Delete!");
        }
    }
}

impl AsRef<pancurses::Window> for Ui {
    fn as_ref(&self) -> &pancurses::Window {
        &self.window
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new(
            Arc::new(pancurses::initscr()),
            Border::new_unicode(),
            ListPanelState::default(),
            FocusPanelState::default(),
            PromptState::default(),
            Bar,
            false,
            Formatter::default(),
            PathBuf::new()
        )
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
