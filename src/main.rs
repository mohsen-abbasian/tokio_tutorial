// use hello_tokio function
// use tokio_toturial::hello_tokio;

// use say_hello_world function
// use tokio_toturial::say_hello_world;

// use spawning_main function
use tokio_toturial::spawning_main;

#[tokio::main]
async fn main() {
    // run hello_tokio function
    // let _ = hello_tokio().await;

    // run say_hello_world function
    // let _ = say_hello_world().await;

    // run spawning_main function
    spawning_main().await;

}
