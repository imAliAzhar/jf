pub use crossterm::event::KeyCode;
use crossterm::event::{
    Event as CrosstermEvent, EventStream as CrosstermEventStream, KeyEvent, KeyEventKind,
};
use futures::StreamExt;
use tracing::debug;

use crate::signal::CloseSignal;

#[derive(Debug)]
pub enum Event {
    KeyPress(KeyCode),

    Unknown,
}

pub struct EventStream {
    terminal_event_stream: CrosstermEventStream,
    close_signal: CloseSignal,
}

impl EventStream {
    pub fn new(close_signal: CloseSignal) -> Self {
        let terminal_event_stream = CrosstermEventStream::new();
        Self {
            close_signal,
            terminal_event_stream,
        }
    }

    pub async fn recv(&mut self) -> Option<Event> {
        tokio::select! {
            Some(Ok(event)) = self.terminal_event_stream.next() => {
                match event {
                    CrosstermEvent::Key( key @ KeyEvent { kind: KeyEventKind::Press, ..  }) => {
                        let event = Event::KeyPress(key.code);
                        debug!("Received event: {event:?}");

                        Some(event)
                    },

                    event => {
                        debug!("Unhandled Crossterm event: {event:?}");
                        Some(Event::Unknown)
                    }
                }
            }

            _ = self.close_signal.recv() => {
                    debug!("Closing event stream...");
                    None
            },
        }
    }
}
