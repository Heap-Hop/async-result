use futures_channel::oneshot::{self, Canceled};
use std::future::Future;

pub struct Completer<T> {
    tx: oneshot::Sender<T>,
}

impl<T> Completer<T> {
    pub fn try_complete(self, res: T) -> Result<(), T> {
        self.tx.send(res)
    }

    pub fn complete(self, res: T) {
        self.try_complete(res).ok();
    }
}

pub struct AsyncResult {}

impl AsyncResult {
    pub async fn with<T>(f: impl FnOnce(Completer<T>)) -> Result<T, Canceled> {
        let (tx, rx) = oneshot::channel();
        f(Completer { tx });
        rx.await
    }

    pub async fn with_async<T, F: Future<Output = ()>>(
        f: impl FnOnce(Completer<T>) -> F,
    ) -> Result<T, Canceled> {
        let (tx, rx) = oneshot::channel();
        f(Completer { tx }).await;
        rx.await
    }

    pub fn new_split<T>() -> (Completer<T>, impl Future<Output = Result<T, Canceled>>) {
        let (tx, rx) = oneshot::channel();
        let fut = async { rx.await };
        (Completer { tx }, fut)
    }
}
