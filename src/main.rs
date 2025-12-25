use std::{
    sync::mpsc::{Receiver, Sender, channel},
    thread::JoinHandle,
};

#[derive(Clone)]
struct Mailbox<T>(Sender<T>);

struct Task<M, R> {
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

macro_rules! proc {
    ($($content:tt)*) => {
        spawn_task(move |_receiver| {
            #[allow(unused_macros)]
            macro_rules! recv {
                () => { _receiver.recv().unwrap() }
            }
            $($content)*
        })
    };
}

fn spawn_task<M, R, Func>(func: Func) -> Task<M, R>
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

fn main() {
    let (sender, receiver) = channel::<u32>();
    let task: Task<u32, u32> = proc! {
        let mut counter = 0;
        for _ in 0..10 {
            let val = recv!();

            println!("received {val}");
            counter += val;
            sender.send(counter).unwrap();
        }

        counter
    };

    let next_task: Task<(), u32> = proc! {
        for i in 0..10 {
            task.send(i);
            let current_counter = receiver.recv().unwrap();
            println!("current counter: {current_counter}");
        }

        task.join()
    };

    let result = next_task.join();
    println!("OH YES! {result}")
}
