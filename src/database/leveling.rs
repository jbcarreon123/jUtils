
use bson::Document;
use mongodb::{bson::doc, options::UpdateOptions};
use serde::{Deserialize, Serialize};
use crate::types::Context;

use super::load_db;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Leveling {
    pub guild_id: String,
    pub levels: Vec<UserLevel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLevel {
    pub user_id: String,
    pub level: i64,
    pub xp: i64,
}

impl UserLevel {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            level: 0,
            xp: 0,
        }
    }

    pub fn add_xp(&mut self, xp: i64) {
        self.xp = self.xp.checked_add(xp).expect("XP overflow");
    }

    pub fn compute_level(&mut self) {
        let mut level = 0;
        while let Some(required_xp) = (5 * level as i64).checked_pow(2)
            .and_then(|v| v.checked_add(50 * (level - 1)))
            .and_then(|v| v.checked_add(100)) {
            if self.xp >= required_xp {
                level += 1;
            } else {
                break;
            }
        }
        self.level = level;
    }

    pub fn compute_xp_required(&self) -> i64 {
        (5 * self.level as i64).pow(2) + 50 * (self.level - 1) + 100
    }
}

impl Leveling {
    pub fn new(guild_id: String) -> Self {
        Self {
            guild_id,
            levels: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user_level: UserLevel) {
        if let Some(existing_user) = self.levels.iter_mut().find(|u| u.user_id == user_level.user_id) {
            existing_user.add_xp(user_level.xp);
            existing_user.compute_level();
        } else {
            self.levels.push(user_level);
        }
    }
}

pub async fn get_leveling(guild_id: String) -> Result<Leveling, mongodb::error::Error> {
    let db = load_db().await;
    let collection = db.collection::<Leveling>("leveling");
    let leveling = collection.find_one(doc! {"guild_id": guild_id.clone()}, None).await?;

    if leveling.is_none() {
        let new_leveling = Leveling::new(guild_id.clone());
        save_leveling(new_leveling.clone()).await?;
        Ok(new_leveling)
    } else {
        Ok(leveling.unwrap())
    }
}

pub async fn save_leveling(leveling: Leveling) -> Result<(), mongodb::error::Error> {
    let db = load_db().await;
    let collection = db.collection::<Leveling>("leveling");
    let guild_id = leveling.guild_id.clone();
    let filter = doc! { "guild_id": guild_id.clone() };

    let options = UpdateOptions::builder()
        .upsert(true)
        .build();
    let bson_conf: Document = doc! { "$set": bson::to_document(&leveling)? };

    match collection
        .update_one(filter, bson_conf, options)
        .await {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
}

pub async fn get_user_leveling(guild_id: String, user_id: String) -> Result<UserLevel, mongodb::error::Error> {
    let leveling = get_leveling(guild_id).await?;
    let user_level = leveling.levels.iter().find(|u| u.user_id == user_id).cloned().unwrap_or_else(|| UserLevel::new(user_id));
    Ok(user_level)
}

pub async fn save_user_leveling(guild_id: String, user_level: UserLevel) -> Result<(), mongodb::error::Error> {
    let mut leveling = get_leveling(guild_id).await?;
    leveling.add_user(user_level);
    save_leveling(leveling).await?;
    Ok(())
}

pub async fn add_xp(guild_id: String, user_id: String, xp: i64) -> Result<(), mongodb::error::Error> {
    let mut user_level = get_user_leveling(guild_id.clone(), user_id.clone()).await?;
    user_level.add_xp(xp);
    user_level.compute_level();
    save_user_leveling(guild_id, user_level).await?;
    Ok(())
}

pub async fn get_user_level(guild_id: String, user_id: String) -> Result<i64, mongodb::error::Error> {
    let user_level = get_user_leveling(guild_id, user_id).await?;
    Ok(user_level.level)
}

pub async fn get_user_xp(guild_id: String, user_id: String) -> Result<i64, mongodb::error::Error> {
    let user_level = get_user_leveling(guild_id, user_id).await?;
    Ok(user_level.xp)
}

pub async fn get_user_rank(guild_id: String, user_id: String) -> Result<i64, mongodb::error::Error> {
    let leveling = get_leveling(guild_id).await?;
    let user_level = if let Some(level) = leveling.levels.iter().find(|u| u.user_id == user_id) {
        level
    } else {
        return Err(mongodb::error::Error::custom("Cannot find user"))
    };
    let rank = leveling.levels.iter().filter(|u| u.xp > user_level.xp).count() as i64 + 1;
    Ok(rank)
}

pub async fn get_leaderboard(guild_id: String) -> Result<Vec<UserLevel>, mongodb::error::Error> {
    let leveling = get_leveling(guild_id).await?;
    let mut levels = leveling.levels.clone();
    levels.sort_by(|a, b| b.xp.cmp(&a.xp));
    Ok(levels)
}

pub async fn get_leaderboard_page(guild_id: String, page: i64, page_size: i64) -> Result<Vec<UserLevel>, mongodb::error::Error> {
    let leaderboard = get_leaderboard(guild_id).await?;
    let start = (page - 1) * page_size;
    let end = start + page_size;
    Ok(leaderboard.into_iter().skip(start as usize).take(end as usize).collect())
}