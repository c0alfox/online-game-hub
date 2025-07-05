use crate::*;
use crate::filters::with_clients;

use futures::{FutureExt, StreamExt, stream::SplitStream};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::filters::ws::{Ws, WebSocket};
use warp::{Filter, Reply, Rejection};
use tokio::sync::mpsc;
use uuid::Uuid;

pub fn root(
    clients: &'static Clients
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let root = warp::path("ws");

    let notifs = root
        .and(warp::path("hearts"))
        .and(warp::ws())
        .and(warp::path::end())
        .and(with_clients(clients))
        .and_then(hearts);

    notifs
}

async fn hearts(
    ws: Ws,
    clients: &'static Clients
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Hearts endpoint reached");

    Ok(ws.on_upgrade( async move |socket| {
        info!("Upgrading");
        let (uuid, stream) = connect_client(socket, clients);
        recv_thread(uuid, stream, clients).await;
    }))
}

fn connect_client(
    socket: WebSocket,
    clients: &'static Clients
) -> (Uuid, SplitStream<WebSocket>) {
    // Get Unbounded Streams and join them with sockets
    let (sink, stream) = socket.split();

    let (sender, receiver) = mpsc::unbounded_channel();

    let receiver = UnboundedReceiverStream::new(receiver);
    tokio::task::spawn(receiver.forward(sink).map(|result| {
        if let Err(e) = result {
            error!("Error sending websocket message: {}", e);
        }
    }));

    // Create and store user session
    let mut locked = clients.lock().unwrap();
    let uuid = Uuid::new_v4();

    locked.insert(uuid.clone(), sender);

    info!("clients: {:?}", locked);

    // Return stream for continuous use in the 
    (uuid, stream)
}

async fn recv_thread(uuid: Uuid, mut recv: SplitStream<WebSocket>, clients: &'static Clients) {
    while let Some(body) = recv.next().await {
        if let Ok(content) = body {
            info!("Got: {:?}", content);
            broadcast(&uuid, content, clients).await;
        }
    }

    destroy_client(uuid, clients).await;
    info!("client disconnected");
}

async fn broadcast(sender_uuid: &Uuid, msg: warp::ws::Message, clients: &'static Clients) {
    let locked = clients.lock().unwrap();
    for (uuid, oth) in locked.iter() {
        if uuid != sender_uuid {
            let _ = oth.send(Ok(msg.clone()));
        }
    }
}

async fn destroy_client(uuid: Uuid, clients: &'static Clients) {
    let mut locked = clients.lock().unwrap();
    locked.remove(&uuid);
}