use tokio::sync::broadcast;

pub type HandlerFunction<T> = Box<dyn Fn(T) + Send + Sync + 'static>;

#[derive(Clone)]
pub struct NotificationManager<T> {
    tx: broadcast::Sender<T>,
}

impl<T> NotificationManager<T>
where
    T: Clone + Send + 'static,
{
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<T> {
        self.tx.subscribe()
    }

    pub fn invoke_all_handlers(&self, data: T) -> Result<usize, broadcast::error::SendError<T>> {
        self.tx.send(data)
    }
}

impl<T> Default for NotificationManager<T>
where
    T: Clone + Send + 'static,
{
    fn default() -> Self {
        Self::new(100)
    }
}
