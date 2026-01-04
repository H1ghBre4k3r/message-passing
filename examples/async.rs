use notizia::{AsyncTask, async_proc};
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = channel::<u32>(64);
    let task: AsyncTask<u32, u32> = async_proc! {
        let mut counter = 0;
        for _ in 0..10 {
            let val = recv!();

            println!("received {val}");
            counter += val;
            sender.send(counter).await.unwrap();
        }

        counter
    };

    let next_task: AsyncTask<(), u32> = async_proc! {
        for i in 0..10 {
            task.send(i).await.unwrap();
            let current_counter = receiver.recv().await.unwrap();
            println!("current counter: {current_counter}");
        }

        task.join().await
    };

    let result = next_task.join().await;
    println!("OH YES! {result}")
}
