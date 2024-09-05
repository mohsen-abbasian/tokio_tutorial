// Implementation of the third section of tutorial (Sared state)

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
type Db = Arc<Mutex<HashMap<String, Bytes>>>;
// type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

// fn new_sharded_db(num_shards: usize) -> ShardedDb {
//     let mut db = Vec::with_capacity(num_shards);
//     for _ in 0..num_shards {
//         db.push(Mutex::new(HashMap::new()));
//     }
//     Arc::new(db)
// }

// let shard = db[hash(key) % db.len()].lock().unwrap();
// shard.insert(key, value);

// To run this function you need to use and await main_shared_state functin from this
// module.
pub async fn main_shared_state() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle to the hash map.
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                {
                    let mut db = db.lock().unwrap();
                    db.insert(cmd.key().to_string(), cmd.value().clone());
                }
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
