// Implementation of the first tutorial (Hello Tokio).

use mini_redis::{client, Result};

// To run this function you need to use and await hello_tokio functin from this
// module. Make sure the Mini-Redis server is running in a separate terminal window.
// by running this tutorial you should see `got value from the server; result=Some(b"world")` in the terminal.
pub async fn hello_tokio() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}

async fn say_world() {
    print!("World");
}

// To run this function you need to use and await say_hello_world functin from this
// module.
// by running this tutorial you should see `Hello World` in the terminal.
pub async fn say_hello_world() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This print! comes first
    print!("Hello ");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}
