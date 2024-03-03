use std::{thread::sleep, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

#[tokio::main]
async fn main() 
{
    let listener = TcpListener::bind("192.168.29.197:2345").await.unwrap();

     tokio::spawn( async move {
        loop {
            println!("Running");
            match listener.accept().await {
                Ok(mut w) => {
                    println!(" Jesus Christ this is working {:?} {}", w.0, w.1);
                    let mut buffer = [0u8; 1024];
                    w.0.read(&mut buffer).await.unwrap();
                    println!("{}", String::from_utf8_lossy(buffer.as_ref()));
                }
                Err(e) => {
                    println!("This is e {}", e);
                }
            }
        }
    });

    println!("Running main thread");

    let mut con = TcpStream::connect("192.168.29.197:2345").await.unwrap();

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(1));
            con.write("Hello World".as_bytes()).await.unwrap();
            con.flush().await.unwrap();
        }
    });
     
    loop{};
}
