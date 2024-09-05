// Implementation of the fifth section of tutorial (I/O)
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

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

pub async fn main_io_3() -> io::Result<()> {
    let mut f = File::create("./files/bar2.txt").await?;

    // Write some prefix of the byte string, but not necessarily all of it
    let n = f.write_all(b"some bytes").await?;

    println!("Wrote the first all bytes of 'some bytes'.");    
    Ok(())
}

pub async fn main_io_4() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("./files/bar3.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    println!("main_io_4 Done!");
    Ok(())
}

pub async fn main_echo_server_copy() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}