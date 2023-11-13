use ratatui::prelude::{Constraint, Layout};
use std::fs;

use crate::{
    actions::Action,
    components::{counter::Counter, explorer::Explorer, Component},
    dispatch,
    events::KeyCode,
    store::State,
    terminal::Frame,
};

pub struct App {}

impl App {
    pub fn new() -> Self {
        let dirs = fs::read_dir(".")
            .map_or_else(|_| Vec::new(), |dir| dir.filter_map(Result::ok).collect());

        dispatch!(Action::SetDirs(dirs));

        Self {}
    }

    pub fn on_key_press(&self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('j') => dispatch!(Action::Increment),
            KeyCode::Char('k') => dispatch!(Action::Decrement),
            KeyCode::Char('r') => dispatch!(Action::Reset),
            KeyCode::Char('q') => dispatch!(Action::Quit),

            _ => {}
        }
    }

    pub fn render(&self, frame: &mut Frame, state: &State) {
        let size = frame.size();

        let [counter_area, explorer_area] = *Layout::default()
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(size)
        else {
            panic!("Could not split layout")
        };

        Counter(frame, counter_area, state);
        Explorer::new().render(frame, explorer_area, state);
    }
}
