use ratatui::prelude::Rect;

use crate::{store::State, terminal::Frame};

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect, state: &State);
}
