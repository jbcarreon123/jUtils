use bson::Document;
use mongodb::{bson::doc, options::UpdateOptions};
use serde::{Deserialize, Serialize};

use super::load_db;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timezones {
    pub user_id: String,
    pub timezone: String,
}

impl Timezones {
    pub fn new(user_id: String, timezone: String) -> Self {
        Timezones {
            user_id,
            timezone,
        }
    }
}

pub async fn get_user_timezone(
    user_id: String,
) -> Result<Option<Timezones>, mongodb::error::Error> {
    let db = load_db().await;
    let collection = db.collection::<Timezones>("timezones");
    let filter = doc! { "user_id": user_id.clone() };
    let timezone = collection.find_one(filter, None).await?;
    Ok(timezone)
}

pub async fn set_user_timezone(
    user_id: String,
    timezone: String,
) -> Result<(), mongodb::error::Error> {
    let db = load_db().await;
    let collection = db.collection::<Timezones>("timezones");
    let filter = doc! { "user_id": user_id.clone() };

    let options = UpdateOptions::builder()
        .upsert(true)
        .build();

    let timezone = Timezones::new(user_id, timezone);
    let bson_tz: Document = bson::to_document(&timezone).unwrap();
    
    match collection
        .update_one(filter, bson_tz, options)
        .await {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
}