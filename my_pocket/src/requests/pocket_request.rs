use serde::{Deserialize, Serialize};
use crate::models::tag_model::Tag;

#[derive(Serialize, Deserialize, Debug)]
pub struct PocketRequest {
    pub description: String,
    pub link: String,
    pub tags: Vec<Tag>,
}
