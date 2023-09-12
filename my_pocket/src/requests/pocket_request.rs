use serde::{Deserialize, Serialize};
use crate::models::{tag_model::Tag, pocket_model::Pocket};

#[derive(Serialize, Deserialize, Debug)]
pub struct PocketRequest {
    pub description: String,
    pub link: String,
    pub tags: Vec<Tag>,
}

impl PocketRequest {
  pub fn into_pocket(self) -> Pocket {
    Pocket {
      id: None,
      description: self.description,
      link: self.link,
      tags: self.tags,
      created_at: chrono::Utc::now(),
    }
  }
}
