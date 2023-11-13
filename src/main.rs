mod actions;
mod app;
mod components;
mod dispatch;
mod events;
mod signal;
mod store;
mod terminal;
mod tracing;

use color_eyre::Result;
use std::sync::Arc;

use actions::Action;
use app::App;
use dispatch::ActionStream;
use events::{Event, EventStream};
use signal::Signal;
use store::State;
use terminal::Terminal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init()?;
    info!("App start");

    let signal = Signal::new();
    let mut action_stream = ActionStream::new(signal.close_signal());
    let mut event_stream = EventStream::new(signal.close_signal());

    let mut state = State::default();
    let app = Arc::new(App::new());

    let app_clone = app.clone();

    tokio::spawn(async move {
        while let Some(event) = event_stream.recv().await {
            if let Event::KeyPress(key_code) = event {
                app_clone.on_key_press(key_code)
            }
        }
    });

    let mut terminal = Terminal::init()?;

    terminal.draw(|f| {
        app.render(f, &state);
    })?;

    while let Some(action) = action_stream.next().await {
        if let Action::Quit = action {
            signal.close();
        }

        state.update(action);
        terminal
            .draw(|f| {
                app.render(f, &state);
            })
            .ok();
    }

    Ok(())
}
