mod events;
mod message;
mod renderer;
mod store;
mod terminal;
mod tracing;

use color_eyre::Result;
use crossterm::event::KeyCode;
use events::{Event, EventStream};
// use event_handler::EventHandler;
use ::tracing::info;
use renderer::Renderer;
use store::Model;
use terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init()?;
    info!("App start");

    let renderer = Renderer::new();
    // let event_handler = EventHandler::new();
    let mut model = Model::default();

    let mut event_stream = EventStream::new();

    let mut terminal = Terminal::init()?;

    terminal.draw(|f| {
        renderer.render(&model, f);
    })?;

    while let Some(event) = event_stream.recv().await {
        match event {
            Event::Key(KeyCode::Char('j')) => model.update(message::Message::Increment),
            Event::Key(KeyCode::Char('k')) => model.update(message::Message::Decrement),
            Event::Key(KeyCode::Char('r')) => model.update(message::Message::Reset),
            Event::Key(KeyCode::Char('q')) => event_stream.close().await,

            _ => {}
        }

        terminal.draw(|f| {
            renderer.render(&model, f);
        })?;
    }

    Ok(())
}
