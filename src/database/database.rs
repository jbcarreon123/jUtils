use mongodb::{
    Client, Database
};
use crate::CONFIG;

pub async fn load_db() -> Database {
    let c = Client::with_uri_str(CONFIG.database.connection_string.clone()).await.expect("Can't connect to the MongoDB database!");
    c.database(&CONFIG.database.db)
}