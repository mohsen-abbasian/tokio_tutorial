// Implementation of the fifth section of tutorial (I/O)
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

pub async fn main_io() -> io::Result<()> {
    let mut f = File::open("./files/foo.txt").await?;
    let mut buffer = [0; 10];
    
    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("Tehe bytes: {:?}", &buffer[..n]);
    Ok(())
}

pub async fn main_io_1() -> io::Result<()> {
    let mut f = File::open("./files/foo.txt").await?;
    let mut buffer = Vec::new();

    // read 
    f.read_to_end(&mut buffer).await?;
    println!("The bytes: {:?}", &buffer);
    Ok(())
}

pub async fn main_io_2() -> io::Result<()> {
    let mut f = File::create("./files/bar.txt").await?;

    // Write some prefix of the byte string, but not necessarily all of it
    let n = f.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);    
    Ok(())
}