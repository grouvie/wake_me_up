use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

use crate::{
    database::data::{client::Client, create_pool::create_pool},
    error::MyResult,
    proto::wake_up::WakeUpProto,
};

#[derive(Clone)]
pub struct ModelController {
    pub client: Client,
    pub connected_clients: Arc<Mutex<HashMap<i32, Sender<WakeUpProto>>>>,
    pub wake_up_sender: Arc<Mutex<HashMap<i32, Sender<bool>>>>,
}

impl ModelController {
    pub async fn new() -> MyResult<Self> {
        let connected_clients = Arc::new(Mutex::new(HashMap::new()));
        let wake_up_sender = Arc::new(Mutex::new(HashMap::new()));
        let pool = create_pool().await?;
        let client = Client::new(pool).await;
        Ok(Self {
            client,
            connected_clients,
            wake_up_sender,
        })
    }
}
