use tokio::net::{TcpListener, TcpStream};
use tokio;
use async_tungstenite;

use std::{
    io::Error as IoError,
    env,
    net::SocketAddr,
    collections::HashMap,
    sync::{Arc, Mutex},
};


use futures::prelude::*;
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future, pin_mut,
};

use async_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr){
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = async_tungstenite::tokio::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming
        .try_filter(|msg| {
            // Broadcasting a Close message from one client
            // will close the other clients.
            future::ready(!msg.is_close())
        })
        .try_for_each(|msg| {
            println!(
                "Received a message from {}: {}",
                addr,
                msg.to_text().unwrap()
            );
            let peers = peer_map.lock().unwrap();

            // We want to broadcast the message to everyone except ourselves.
            let broadcast_recipients = peers
                .iter()
                .filter(|(peer_addr, _)| peer_addr != &&addr)
                .map(|(_, ws_sink)| ws_sink);

            for recp in broadcast_recipients {
                recp.unbounded_send(msg.clone()).unwrap();
            }

            future::ok(())
        });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
} 

#[tokio::main]
async fn main() -> Result<(), IoError>{
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:80".to_string());


    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}