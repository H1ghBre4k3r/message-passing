use std::{
    sync::mpsc::{Receiver, RecvError, RecvTimeoutError, SendError, Sender, channel},
    thread::JoinHandle,
    time::Duration,
};

pub struct Mailbox<T> {
    receiver: Receiver<T>,
}

impl<T> Mailbox<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        Mailbox { receiver }
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }

    pub fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }
}

pub struct Task<M, R> {
    sender: Sender<M>,
    handle: JoinHandle<R>,
}

impl<T, R> Task<T, R>
where
    T: Clone,
{
    pub fn send(&self, payload: T) -> Result<(), SendError<T>> {
        self.sender.send(payload)
    }

    pub fn join(self) -> std::thread::Result<R> {
        self.handle.join()
    }
}

#[macro_export]
macro_rules! proc {
    ($($content:tt)*) => {
        notizia::spawn_task(move |__mb| {
            #[allow(unused_macros)]
            macro_rules! recv {
                () => { __mb.recv().unwrap() }
            }
            $($content)*
        })
    };
}

pub fn spawn_task<M, R, Func>(func: Func) -> Task<M, R>
where
    M: Send + 'static,
    R: Send + 'static,
    Func: FnOnce(Mailbox<M>) -> R + Send + 'static,
{
    let (sender, receiver) = channel::<M>();
    let mailbox = Mailbox::new(receiver);
    let handle = std::thread::spawn(move || func(mailbox));

    Task { sender, handle }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_task_communication() {
        let task = spawn_task(|mailbox| {
            let mut total = 0;
            for _ in 0..3 {
                total += mailbox.recv().unwrap();
            }
            total
        });

        task.send(10).unwrap();
        task.send(20).unwrap();
        task.send(30).unwrap();

        let result = task.join().unwrap();
        assert_eq!(result, 60);
    }

    #[test]
    fn test_sum_of_five_numbers() {
        let task = spawn_task(|receiver| {
            let mut total = 0;
            for _ in 0..5 {
                total += receiver.recv().unwrap();
            }
            total
        });

        for i in 1..=5 {
            task.send(i).unwrap();
        }

        let result = task.join().unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_spawn_task_with_multiple_messages() {
        let task = spawn_task(|receiver| {
            let mut sum = 0;
            for _ in 0..3 {
                sum += receiver.recv().unwrap();
            }
            sum
        });

        task.send(5).unwrap();
        task.send(10).unwrap();
        task.send(15).unwrap();

        let result = task.join().unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    fn test_string_messages() {
        let task = spawn_task(|receiver| {
            let mut count = 0;
            for _ in 0..3 {
                receiver.recv().unwrap();
                count += 1;
            }
            count
        });

        task.send("hello".to_string()).unwrap();
        task.send("world".to_string()).unwrap();
        task.send("test".to_string()).unwrap();

        let result = task.join().unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_task_returns_vec() {
        let task = spawn_task(|receiver| {
            let mut values = Vec::new();
            for _ in 0..5 {
                let val = receiver.recv().unwrap();
                values.push(val);
            }
            values
        });

        for i in 1..=5 {
            task.send(i).unwrap();
        }

        let result = task.join().unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_empty_task() {
        let task = spawn_task::<(), u32, _>(|_receiver| 42);

        let result = task.join().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_different_number_types() {
        let task = spawn_task(|receiver| {
            let mut sum: i64 = 0;
            for _ in 0..3 {
                let val: i32 = receiver.recv().unwrap();
                sum += val as i64;
            }
            sum
        });

        task.send(100).unwrap();
        task.send(200).unwrap();
        task.send(300).unwrap();

        let result = task.join().unwrap();
        assert_eq!(result, 600);
    }
}
