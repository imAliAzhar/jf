use crossterm::event::{
    Event as CrosstermEvent, EventStream as CrosstermEventStream, KeyCode, KeyEvent, KeyEventKind,
};
use futures::StreamExt;
use tokio::sync::mpsc::{self, Sender, UnboundedReceiver};
use tracing::{debug, info};

pub enum Event {
    Key(KeyCode),
    UnhandledEvent(CrosstermEvent),
}

pub struct EventStream {
    rx: UnboundedReceiver<Event>,
    stop_tx: Sender<()>,
}

impl EventStream {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let (stop_tx, mut stop_rx) = mpsc::channel(1);

        tokio::spawn(async move {
            let mut reader = CrosstermEventStream::new();
            info!("Starting event stream");

            loop {
                tokio::select! {
                    _ = stop_rx.recv() => break,

                    Some(Ok(event)) = reader.next() => {
                        let event = match event {

                        CrosstermEvent::Key( key @ KeyEvent { kind: KeyEventKind::Press, ..  }) => Event::Key(key.code),

                        event => { debug!("Unhandled Crossterm event: {event:?}"); Event::UnhandledEvent(event) } };

                    if tx.send(event).is_err() { break; } }

                }
            }
        });

        Self { rx, stop_tx }
    }

    pub async fn recv(&mut self) -> Option<Event> {
        self.rx.recv().await
    }

    pub async fn close(&self) {
        info!("Closing event stream");
        self.stop_tx.send(()).await.ok();
    }
}
