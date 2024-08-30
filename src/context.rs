use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
pub struct Data {
    pub saved_rolls: Arc<Mutex<HashMap<String, String>>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
