use ratatui::widgets::Paragraph;

use crate::{store::Model, terminal::Frame};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, model: &Model, f: &mut Frame) {
        f.render_widget(
            Paragraph::new(format!("Counter: {}", model.counter)),
            f.size(),
        );
    }
}
