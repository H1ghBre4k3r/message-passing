use std::sync::mpsc::channel;

use notizia::{Task, proc};

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
            task.send(i).unwrap();
            let current_counter = receiver.recv().unwrap();
            println!("current counter: {current_counter}");
        }

        task.join().expect("Task should be able to join")
    };

    let result = next_task.join().expect("Task should be able to join");
    println!("OH YES! {result}")
}
