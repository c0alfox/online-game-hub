use crate::*;
use warp::{path::FullPath, Filter};

pub fn log_request() -> impl Filter<Extract = (), Error = std::convert::Infallible> + Clone {
    warp::path::full()
        .map(|p: FullPath| {
            info!("REQ: {}", p.as_str());
        })
        .untuple_one()
}

pub fn with_clients(
    clients: &'static Clients
) -> impl Filter<Extract = (&'static Clients,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || clients)
}