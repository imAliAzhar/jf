use ratatui::{
    prelude::Rect,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{components::Component, store::State, terminal::Frame};

pub struct Explorer {}

impl Explorer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Explorer {
    fn render(&self, frame: &mut Frame, area: Rect, state: &State) {
        let dir_div = Block::default().title("alif").borders(Borders::ALL);

        let list_items = state
            .dirs
            .iter()
            .map(|d| d.file_name().into_string().unwrap())
            .map(ListItem::new)
            .collect::<Vec<ListItem>>();

        let list = List::new(list_items).block(dir_div);

        frame.render_widget(list, area);
    }
}
