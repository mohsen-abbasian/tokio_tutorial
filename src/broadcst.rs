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
    let subscriber = client.subscribe(vec!["Numbers".to_string()]).await?;
    
    // get all results
    // let messages = subscriber.into_stream();

    // get the first three results
    // let messages = subscriber.into_stream().take(3);

    // get the first three results with len of results equals to 1
    // let messages = subscriber.into_stream().filter(|msg| match msg {
    //     Ok(msg) if msg.content.len() == 1 => true,
    //     _ => false,
    // }).take(3);

    // get the first three results with len of results equals to 1 and unwrap message content 
    let messages = subscriber.into_stream().filter(|msg| match msg {
        Ok(msg) if msg.content.len() == 1 => true,
        _ => false,
    }).map(|msg| msg.unwrap().content).take(3);

    tokio::pin!(messages);
    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }
    Ok(())
}


// To run this function run redis-mini-server in a separated retminal
pub async fn main_broadcast() -> mini_redis::Result<()> {
    tokio::spawn(async {
        let _ = publish().await;
    });

    subscribe().await?;

    Ok(())
}