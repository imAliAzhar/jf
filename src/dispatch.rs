pub use crossterm::event::KeyCode;
use std::sync::OnceLock;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tracing::debug;

use crate::{actions::Action, signal::CloseSignal};

// Dispatcher is static to make dispatching from components more ergonomic
static DISPATCHER: OnceLock<UnboundedSender<Action>> = OnceLock::new();

#[macro_export]
macro_rules! dispatch {
    ($action:expr) => {
        $crate::dispatch::ActionStream::dispatch($action)
    };
}

pub struct ActionStream {
    rx: UnboundedReceiver<Action>,
    close_signal: CloseSignal,
}

impl ActionStream {
    pub fn new(close_signal: CloseSignal) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        DISPATCHER
            .set(tx)
            .expect("Dispatcher should be available for initialization");

        Self { rx, close_signal }
    }

    pub async fn next(&mut self) -> Option<Action> {
        tokio::select! {
            action = self.rx.recv() => action,

            _ = self.close_signal.recv() => {
                    debug!("Closing action stream...");
                    None
            },

        }
    }

    pub fn dispatch(action: Action) {
        debug!("Dispatching action: {action:?}");

        let tx = DISPATCHER.get().expect("Dispatcher to be initialized");
        tx.send(action).expect("Action to be sent");
    }
}
