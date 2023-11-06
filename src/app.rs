use ratatui::prelude::{Constraint, Layout};

use crate::{
    components::{counter::Counter, explorer::Explorer, Component},
    store::State,
    terminal::Frame,
};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
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
