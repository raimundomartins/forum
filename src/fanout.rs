use futures::{future::join_all, Future};
use tokio::sync::mpsc;

pub struct FanOut<T> {
    //in_tx: mpsc::Sender<T>,
    //in_rx: mpsc::Receiver<T>,
    out_tx: Vec<mpsc::Sender<T>>,
}

impl<T> FanOut<T> {
    pub fn new(_backpressure: usize) -> Self {
        //let (in_tx, in_rx) = mpsc::channel(backpressure);
        Self {
            //in_tx,
            //in_rx,
            out_tx: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, tx: mpsc::Sender<T>) {
        self.out_tx.push(tx);
    }

    fn tx_cleanup<I>(&mut self, iter: I)
    where
        I: Iterator<Item = Result<(), mpsc::error::SendError<T>>>,
    {
        for (i, _) in iter.enumerate().filter(|(_, r)| r.is_err()) {
            self.out_tx.remove(i);
        }
    }

    /*pub fn inlet(&mut self) -> mpsc::Sender<T> {
        self.in_tx.clone()
    }*/
}

impl<T: Clone> FanOut<T> {
    pub async fn send(&mut self, data: &T) {
        let res = join_all(self.out_tx.iter().map(|tx| tx.send(data.clone()))).await;
        self.tx_cleanup(res.into_iter());
    }

    /*pub async fn dispatch(&mut self) {
        let data = self.in_rx.recv().await.unwrap();
        self.send(&data).await;
    }

    pub fn into_open_task(mut self) -> (impl Future<Output = ()>, mpsc::Sender<mpsc::Sender<T>>) {
        let (new_sub_tx, mut new_sub_rx) = mpsc::channel(16);
        (
            async move {
                loop {
                    tokio::select! {
                        _ = self.dispatch() => {}
                        new_sub = new_sub_rx.recv() => {
                            if let Some(new_sub) = new_sub {
                                self.subscribe(new_sub);
                            } else {
                                break;
                            }
                        }
                    }
                }
                loop {
                    self.dispatch().await;
                }
            },
            new_sub_tx,
        )
    }

    pub fn into_task(mut self) -> impl Future<Output = ()> {
        async move {
            loop {
                self.dispatch().await;
            }
        }
    }*/
}
