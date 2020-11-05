use tokio::net::{TcpListener, TcpStream};
use tokio;
use async_tungstenite;

use std::{
    io::Error as IoError,
    env,
    net::SocketAddr
};



async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr){
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = async_tungstenite::tokio::accept_async(raw_stream) ;

    

    
    
    
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