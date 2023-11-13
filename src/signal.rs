use tokio_util::sync::{CancellationToken, WaitForCancellationFuture};

#[derive(Clone)]
pub struct CloseSignal(CancellationToken);

impl CloseSignal {
    pub fn recv(&self) -> WaitForCancellationFuture {
        self.0.cancelled()
    }
}

pub struct Signal {
    close: CloseSignal,
}

impl Signal {
    pub fn new() -> Self {
        let close = CloseSignal(CancellationToken::new());

        Self { close }
    }

    pub fn close_signal(&self) -> CloseSignal {
        self.close.clone()
    }

    pub fn close(&self) {
        self.close.0.cancel()
    }
}
