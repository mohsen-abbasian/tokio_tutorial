// use hello_tokio function
// use tokio_toturial::hello_tokio;

// use say_hello_world function
// use tokio_toturial::say_hello_world;

// use spawning_main function
// use tokio_toturial::spawning_main;

// use spawning_main_concurrent function
// use tokio_toturial::spawning_main_concurrent;

// use main_shared_state function
// use tokio_toturial::main_shared_state;

// use main_channel function
// use tokio_toturial::main_channel;

// use main_io function
use tokio_toturial::main_io;

#[tokio::main]
async fn main() {
    // run hello_tokio function
    // let _ = hello_tokio().await;

    // run say_hello_world function
    // let _ = say_hello_world().await;

    // run spawning_main function
    // spawning_main().await;

    // run spawning_main_concurrent function
    // spawning_main_concurrent().await;

    // run main_shared_state function
    // main_shared_state().await;

    // run main_channel function
    // main_channel().await;

    // run main_io function
    let _ = main_io().await;
}
