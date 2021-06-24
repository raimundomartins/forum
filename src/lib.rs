use std::collections::HashMap;
use tokio::sync::{mpsc, watch};

pub mod fanout;

use fanout::FanOut;

pub struct Forum {
    backpressure: usize,
    topics: HashMap<core::any::TypeId, Box<dyn core::any::Any>>,
}

impl Forum {
    pub fn new(backpressure: usize) -> Self {
        Self {
            backpressure,
            topics: HashMap::default(),
        }
    }

    pub async fn publish<T: 'static + Clone>(&mut self, data: &T) {
        let topic = core::any::TypeId::of::<T>();
        if let Some(fanout) = self.topics.get_mut(&topic) {
            fanout.downcast_mut::<FanOut<T>>().unwrap().send(data).await;
        }
    }

    pub fn subscribe<T: 'static>(&mut self, tx: mpsc::Sender<T>) {
        let topic = core::any::TypeId::of::<T>();
        match self.topics.entry(topic) {
            std::collections::hash_map::Entry::Occupied(mut entry) => entry
                .get_mut()
                .downcast_mut::<FanOut<T>>()
                .unwrap()
                .subscribe(tx),
            std::collections::hash_map::Entry::Vacant(entry) => {
                let mut fanout = FanOut::new(self.backpressure);
                fanout.subscribe(tx);
                entry.insert(Box::new(fanout));
            }
        }
    }
}

/*pub struct Stage<T> {
    tx: watch::Sender<T>,
    rx: watch::Receiver<T>,
}

impl<T> Stage<T> {
    fn new(initial: T) -> Self {
        let (tx, rx) = watch::channel(initial);
        Self { tx, rx }
    }

    fn send(&mut self, t: T) -> Result<(), watch::error::SendError<T>> {
        self.tx.send(t)
    }

    fn new_viewer(&self) -> watch::Receiver<T> {
        self.rx.clone()
    }
}

impl<T: Default> Default for Stage<T> {
    fn default() -> Self {
        let (tx, rx) = watch::channel(T::default());
        Self { tx, rx }
    }
}*/
