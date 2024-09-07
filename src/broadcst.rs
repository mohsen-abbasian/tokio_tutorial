use std::marker::PhantomData;

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
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber.into_stream();

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}