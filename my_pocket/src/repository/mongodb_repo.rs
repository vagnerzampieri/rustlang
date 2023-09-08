use std::env;
use dotenv::dotenv;

use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    Client, Collection
};

use crate::models::tag_model::Tag;
use crate::models::pocket_model::Pocket;

pub struct MongoRepo {
    tag_collection: Collection<Tag>,
    pocket_collection: Collection<Pocket>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();

        let uri = match env::var("MONGO_URI") {
            Ok(val) => val,
            Err(_) => panic!("MONGO_URI must be set"),
        };

        let db_name = match env::var("MONGO_DB") {
            Ok(val) => val,
            Err(_) => panic!("MONGO_DB must be set"),
        };

        let client = Client::with_uri_str(&uri).await.unwrap();
        let db = client.database(&db_name);

        MongoRepo {
            tag_collection: db.collection("Tag"),
            pocket_collection: db.collection("Pocket"),
        }
    }

    pub async fn create_tag(&self, tag: Tag) -> Result<InsertOneResult, Error> {
        Ok(self.tag_collection
            .insert_one(tag, None)
            .await
            .ok()
            .expect("Error creating tag"))
    }

    pub async fn create_pocket(&self, pocket: Pocket) -> Result<InsertOneResult, Error> {
        Ok(self.pocket_collection
            .insert_one(pocket, None)
            .await
            .ok()
            .expect("Error creating pocket"))
    }
}
