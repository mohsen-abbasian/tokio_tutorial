// Implementation of the fifth section of tutorial (I/O)
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

pub async fn main_io() -> io::Result<()> {
    let mut f = File::open("./files/foo.txt").await?;
    let mut buffer = [0; 10];
    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("Tehe bytes: {:?}", &buffer[..n]);
    Ok(())
}
