use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub type Clients = Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>;
