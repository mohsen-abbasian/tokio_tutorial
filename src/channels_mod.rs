use bytes::Bytes;
use mini_redis::client;

// pub async fn main_channel() {
//     // Stablish a connection to server
//     let mut client = client::connect("127.0.0.1:6379").await.unwrap();

//     // Spawn two tasks, one gets a key, the other sets a key
//     let t1 = tokio::spawn(async {
//         let res = client.get("foo").await;
//     });

//     let t2 = tokio::spawn(async {
//         client.set("foo", "bar".into()).await;
//     });

//     t1.await.unwrap();
//     t2.await.unwrap();
// }

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }
}