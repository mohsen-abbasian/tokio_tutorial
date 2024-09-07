// Implementation of the ninth section of tutorial (Streams)
use tokio_stream::StreamExt;

pub async fn main_stream() {
    let mut stream = tokio_stream::iter(&[1, 2, 3]);

    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }
}
