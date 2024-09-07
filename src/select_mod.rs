// Implementation of the eighth section of tutorial (Select)
use tokio::sync::oneshot;
use tokio::time::{Duration, sleep};

pub async fn main_select() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2  compeleted first with {:?}", val);
        }
    }
}