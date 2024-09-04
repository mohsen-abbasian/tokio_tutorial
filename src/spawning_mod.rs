// Implementation of the second section of tutorial (spawning)

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

async fn process(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("uninplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}

// To run this function you need to use and await spawning_main functin from this
// module.
// by running this tutorial and run `mini-redis-cli get foo` in the separated terminal, in this terminal
// you should see `GOT: Array([Bulk(b"get"), Bulk(b"foo")])` and in client terminal you should see
// `Error: "uninplemented"`.
pub async fn spawning_main() {
    // Bind the listener to the addres
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

// To run this function you need to use and await spawning_main_concurrent functin from this
// module.
// by running this tutorial and run `mini-redis-cli get foo` in the separated terminal, in this terminal
// you should see `GOT: Array([Bulk(b"get"), Bulk(b"foo")])` and in client terminal you should see
// `Error: "uninplemented"`.
pub async fn spawning_main_concurrent() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
