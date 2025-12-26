use tokio::{
    sync::mpsc::{Receiver, Sender, channel},
    task::JoinHandle,
};

#[derive(Clone)]
struct AsyncMailbox<T>(Sender<T>);

pub struct AsyncTask<M, R> {
    mailbox: AsyncMailbox<M>,
    handle: JoinHandle<R>,
}

impl<T, R> AsyncTask<T, R>
where
    T: Clone,
{
    pub async fn send(&self, payload: T) {
        self.mailbox.0.send(payload).await.unwrap()
    }

    pub async fn join(self) -> R {
        self.handle.await.unwrap()
    }
}

#[macro_export]
macro_rules! async_proc {
    ($($content:tt)*) => {
        notizia::spawn_async_task(move |mut _receiver| async move {
            #[allow(unused_macros)]
            macro_rules! recv {
                () => { _receiver.recv().await.unwrap() }
            }
            $($content)*
        })
    };
}

pub fn spawn_async_task<M, R, Output, Func>(func: Func) -> AsyncTask<M, Output>
where
    M: Send + 'static,
    R: Send + 'static + Future<Output = Output>,
    Output: Send + 'static,
    Func: FnOnce(Receiver<M>) -> R + Send + 'static,
{
    let (sender, receiver) = channel::<M>(64);
    let mb = AsyncMailbox(sender);
    let handle = tokio::spawn(func(receiver));

    AsyncTask {
        mailbox: mb,
        handle,
    }
}
