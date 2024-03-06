use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Embed {
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub thumbnail: Option<String>,
    pub image: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LinkComponent {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageData {
    pub guild_id: u64,
    pub channel_id: u64,
    pub embed: Embed,
    pub link_components: Vec<LinkComponent>,
}

pub fn get_msg_data_from_json_str(json: &str) -> Result<MessageData, serde_json::Error> {
    serde_json::from_str(json)
}