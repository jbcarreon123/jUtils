use std::time::Duration;

use bson::{oid::ObjectId, Bson, DateTime};
use chrono::Utc;
use mongodb::{
    bson::doc, options::{ClientOptions, Credential, FindOneOptions, FindOptions, UpdateOptions}, Client, Collection, Database
};
use serde::{Deserialize, Serialize};
use serenity::{all::{CreateEmbed, User, UserId}, futures::StreamExt};
use crate::{utils::generate_id, CONFIG};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Warn {
    pub id: String,
    pub issuer: u64,
    pub user: u64,
    pub reason: String,
    pub duration: DateTime,
    pub resolved: bool
}

impl Warn {
    pub fn new(issuer: u64, user: u64, reason: String, dur: chrono::DateTime<Utc>) -> Self {
        let resolved = false;
        let id = generate_id(6);
        let duration = DateTime::from_chrono::<Utc>(dur);
        Warn {
            id,
            issuer,
            user,
            reason,
            duration,
            resolved
        }
    }
}

pub trait WarnEmbedHelper {
    fn to_embed(&self, user: User) -> CreateEmbed;
}

impl WarnEmbedHelper for Vec<Warn> {
    fn to_embed(&self, user: User) -> CreateEmbed {
        let mut embed = CreateEmbed::default()
            .title(format!("{}'s warns", user.name));
        for warn in self {
            let res = if warn.resolved {
                "Resolved "
            } else {
                ""
            };
            embed = embed.clone().field(
                format!("{}Warn {}", res, warn.id),
                format!("{}\n\nDuration: {}\nissued by <@{}>", warn.reason, warn.duration, warn.issuer),
                true);
        }
        embed
    }
}

pub async fn load_db() -> Database {
    let c = Client::with_uri_str(CONFIG.database.connection_string.clone()).await.expect("Can't connect to the MongoDB database!");
    c.database(&CONFIG.database.db)
}

pub async fn get_warn(
    id: String
) -> Result<Option<Warn>, mongodb::error::Error> {
    let collection = get_warns().await;
    let filter = doc! { "id": id };
    let options = FindOneOptions::default();

    match collection.find_one(filter, options).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

pub async fn get_warns() -> Collection<Warn> {
    let db = load_db().await;
    db.collection::<Warn>("warns")
}

pub async fn get_warnings_by_user(
    user_id: u64
) -> Result<Vec<Warn>, mongodb::error::Error> {
    let collection = get_warns().await;
    let filter = doc! { "user": Bson::from(user_id as i64) };
    let options = FindOptions::default();
    let mut cursor = match collection.find(filter, options).await {
        Ok(res) => res,
        Err(e) => return Err(e.into())
    };

    let mut results: Vec<Warn> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(warn) => {
                results.push(warn);
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(results)
}

pub async fn resolve_warn(id: String) -> Result<(), mongodb::error::Error> {
    let collection = get_warns().await;
    let filter = doc! { "id": id };
    let update_doc = doc! { "$set": { "resolved": true } };
    let options = UpdateOptions::default();

    match collection.update_one(filter, update_doc, options).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn create_warn(issuer: u64, user: u64, reason: String, duration: chrono::DateTime<Utc>) -> Result<String, mongodb::error::Error> {
    let collection = get_warns().await;
    let nw = Warn::new(
        issuer,
        user,
        reason,
        duration
    );
    _ = collection.insert_one(
        nw.clone(),
        None
    ).await;

    Ok(nw.id)
}