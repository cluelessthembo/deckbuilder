use tokio::net::TcpStream;
use tokio::prelude::*;

    
#[tokio::main]
pub async fn net_main(){
    loop {
        update().await;
    }
}

async fn update() {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("created stream");

    let result = stream.write(b"running.. sending messages, you know\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());
}