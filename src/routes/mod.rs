mod wss;

use crate::*;
use crate::filters::log_request;

use std::convert::Infallible;
use warp::{Filter, Reply, Rejection};

pub fn routes(
    clients: &'static Clients
) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let preprocess = log_request();
    let routes = wss::root(clients);
    info!("Routes registered");

    let server = preprocess
        .and(routes)
        .with(warp::cors().allow_any_origin())
        .recover(recover);

    server
}

pub async fn recover(_r: Rejection) -> Result<impl Reply, Infallible> {
    warn!("Route not matched");
    Ok(warp::http::StatusCode::NOT_FOUND)
}
