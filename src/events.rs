use crossterm::event::{
    Event as CrosstermEvent, EventStream as CrosstermEventStream, KeyCode, KeyEvent, KeyEventKind,
};
use futures::StreamExt;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tracing::debug;

pub enum Event {
    Key(KeyCode),
    UnhandledEvent(CrosstermEvent),
}

pub struct EventStream {
    rx: UnboundedReceiver<Event>,
}

impl EventStream {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let mut reader = CrosstermEventStream::new();

            loop {
                if let Some(Ok(event)) = reader.next().await {
                    let event = match event {
                        CrosstermEvent::Key(
                            key @ KeyEvent {
                                kind: KeyEventKind::Press,
                                ..
                            },
                        ) => Event::Key(key.code),

                        event => {
                            debug!("Unhandled Crossterm event: {event:?}");
                            Event::UnhandledEvent(event)
                        }
                    };

                    if tx.send(event).is_err() {
                        break;
                    }
                }
            }
        });

        Self { rx }
    }

    pub async fn recv(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
