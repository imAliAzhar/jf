mod actions;
mod app;
mod components;
mod events;
mod store;
mod terminal;
mod tracing;

use std::fs;

use ::tracing::info;
use app::App;
use color_eyre::Result;
use crossterm::event::KeyCode;
use events::{Event, EventStream};
use store::State;
use terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init()?;
    info!("App start");

    let app = App::new();

    let mut state = State::default();

    let mut event_stream = EventStream::new();

    let mut terminal = Terminal::init()?;

    let dirs = fs::read_dir(".")?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    state.add_dirs(dirs);

    terminal.draw(|f| {
        app.render(f, &state);
    })?;

    while let Some(event) = event_stream.recv().await {
        match event {
            Event::Key(KeyCode::Char('j')) => state.update(actions::Action::Increment),
            Event::Key(KeyCode::Char('k')) => state.update(actions::Action::Decrement),
            Event::Key(KeyCode::Char('r')) => state.update(actions::Action::Reset),
            Event::Key(KeyCode::Char('q')) => event_stream.close().await,

            _ => {}
        }

        terminal.draw(|f| {
            app.render(f, &state);
        })?;
    }

    Ok(())
}
