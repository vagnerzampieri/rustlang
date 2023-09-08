use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use super::tag_model::Tag;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pocket {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub description: String,
    link: String,
    tags: Vec<Tag>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
