use tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio::net::*;
// use futures_util::stream::stream::StreamExt;

use async_tungstenite::tungstenite::Message;

use async_tungstenite;

// use tokio::prelude::*;

use async_tungstenite::tokio as tokio_ts;

use async_tungstenite::tungstenite::Error as TsError;

use std::{
    io::Error as IoError,
    env,
    net::SocketAddr
};

use futures::prelude::*;


async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) -> Result<(), TsError>{
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_ts::accept_async(raw_stream).await.expect("bro");

    
    let (mut outgoing, incoming) = ws_stream.split();

    outgoing.send(Message::Ping(vec!())).await?;

    println!("assigning the callback");
    let _thing  = incoming.try_for_each(|msg| async move{
        println!("running callback");
        match msg{
            Message::Ping(_) => {println!("Pinged by client");
                },
            _=> ()
        }
        
        println!("{:?}", msg.into_text());

        Ok(())
    });

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