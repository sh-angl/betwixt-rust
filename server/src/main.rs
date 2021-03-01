use tokio;
use tokio::net::{TcpListener, TcpStream};

use async_tungstenite::tungstenite::Message;

use async_tungstenite;

use async_tungstenite::tokio as tokio_ts;

use async_tungstenite::tungstenite::Error as TsError;

use std::{
    io::Error as IoError,
    env,
    net::SocketAddr
};


async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) -> Result<(), TsError>{
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_ts::accept_async(raw_stream).await.expect("bro");

    // let (outgoing, incoming) = ws_stream.split();

    
    Ok(())
    
} 


#[tokio::main]
async fn main() -> Result<(), IoError>{
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());


    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);


    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }

    Ok(())
}