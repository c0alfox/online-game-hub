use std::{collections::HashMap, sync::{Arc, Mutex}};
use uuid::Uuid;
use warp::ws::Message;
use tokio::sync::mpsc;

pub type Session = mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>;
pub type Clients = Arc<Mutex<HashMap<Uuid, Session>>>;
