mod filters;
mod macros;
mod routes;
mod types;

use crate::types::Clients;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::boxed::Box;

#[tokio::main]
async fn main() {
    info!("Application started");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let clients: &'static Clients = Box::leak(Box::new(clients));

    let server = routes::routes(clients);
    
    info!("Server started");
    warp::serve(server).run(([0, 0, 0, 0], 8888)).await;
}
