use crossterm::event::{
    Event as CrosstermEvent, EventStream as CrosstermEventStream, KeyCode, KeyEvent, KeyEventKind,
};
use futures::StreamExt;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    oneshot::{self, Sender},
};
use tracing::{debug, info};

pub enum Event {
    Key(KeyCode),
    UnhandledEvent(CrosstermEvent),
}

pub struct EventStream {
    rx: UnboundedReceiver<Event>,
    tx: Option<UnboundedSender<Event>>,
    stop_tx: Option<Sender<()>>,
}

impl EventStream {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let (stop_tx, mut stop_rx) = oneshot::channel();

        let tx_cloned = tx.clone();

        tokio::spawn(async move {
            let mut reader = CrosstermEventStream::new();
            info!("Starting event stream");

            loop {
                tokio::select! {
                    _ = &mut stop_rx => break,

                    Some(Ok(event)) = reader.next() => {
                        let event = match event {

                        CrosstermEvent::Key( key @ KeyEvent { kind: KeyEventKind::Press, ..  }) => Event::Key(key.code),

                        event => { debug!("Unhandled Crossterm event: {event:?}"); Event::UnhandledEvent(event) } };

                    if tx_cloned.send(event).is_err() { break; } }

                }
            }
        });

        Self {
            rx,
            tx: Some(tx),
            stop_tx: Some(stop_tx),
        }
    }

    pub async fn recv(&mut self) -> Option<Event> {
        self.rx.recv().await
    }

    pub async fn close(&mut self) {
        info!("Closing event stream");
        let err = "event stream to not be closed already";

        // Break off the event listening loop
        self.stop_tx.take().expect(err).send(()).ok();

        // Drop the sender so event stream closes
        self.tx.take().expect(err);
    }
}
