use std::{
    sync::mpsc::{Receiver, Sender, channel},
    thread::JoinHandle,
};

#[derive(Clone)]
struct Mailbox<T>(Sender<T>);

pub struct Task<M, R> {
    mailbox: Mailbox<M>,
    handle: JoinHandle<R>,
}

impl<T, R> Task<T, R>
where
    T: Clone,
{
    pub fn send(&self, payload: T) {
        self.mailbox.0.send(payload).unwrap()
    }

    pub fn join(self) -> R {
        self.handle.join().unwrap()
    }
}

#[macro_export]
macro_rules! proc {
    ($($content:tt)*) => {
        message_passing::spawn_task(move |_receiver| {
            #[allow(unused_macros)]
            macro_rules! recv {
                () => { _receiver.recv().unwrap() }
            }
            $($content)*
        })
    };
}

pub fn spawn_task<M, R, Func>(func: Func) -> Task<M, R>
where
    M: Send + 'static,
    R: Send + 'static,
    Func: FnOnce(Receiver<M>) -> R + Send + 'static,
{
    let (sender, receiver) = channel::<M>();
    let mb = Mailbox(sender);
    let handle = std::thread::spawn(move || func(receiver));

    Task {
        mailbox: mb,
        handle,
    }
}
