use ratatui::{prelude::Rect, widgets::Paragraph};

use crate::{store::State, terminal::Frame};

#[allow(non_snake_case)]
pub fn Counter(frame: &mut Frame, area: Rect, state: &State) {
    let counter = Paragraph::new(format!("Counter: {}", state.counter));
    frame.render_widget(counter, area);
}
