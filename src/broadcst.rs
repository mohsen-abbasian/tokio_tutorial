// Implementation of mini-redis broadcast of the ninth section of tutorial (Streams)
use tokio_stream::StreamExt;
use mini_redis::client;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    
    // Publish some data
    client.publish("Numbers", "1".into()).await?;
    client.publish("Numbers", "two".into()).await?;
    client.publish("Numbers", "3".into()).await?;
    client.publish("Numbers", "four".into()).await?;
    client.publish("Numbers", "five".into()).await?;
    client.publish("Numbers", "6".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    println!("0");
    let client = client::connect("127.0.0.1:6379").await?;
    println!("1");
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    println!("2");
    let messages = subscriber.into_stream();
    println!("3");
    tokio::pin!(messages);
    println!("4");
    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }
    println!("5");
    Ok(())
}

pub async fn main_broadcast() -> mini_redis::Result<()> {
    println!("00");
    tokio::spawn(async {
        publish().await;
    });
    println!("01");

    subscribe().await?;

    println!("DONE");

    Ok(())
}