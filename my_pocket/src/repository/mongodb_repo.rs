use std::env;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::InsertOneResult, results::DeleteResult,
    Client, Collection,
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

    pub async fn get_tag(&self, id: &String) -> Result<Tag, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };

        let tag_detail = self.tag_collection
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting tag");

        Ok(tag_detail.unwrap())
    }

    pub async fn update_tag(&self, id: &String, tag: Tag) -> Result<Tag, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };

        Ok(self.tag_collection
            .find_one_and_replace(filter, tag, None)
            .await
            .ok()
            .expect("Error updating tag")
            .unwrap())

    }

    pub async fn delete_tag(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };

        let tag = self.tag_collection
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting tag");

        Ok(tag)
    }

    pub async fn create_pocket(&self, pocket: Pocket) -> Result<InsertOneResult, Error> {
        Ok(self.pocket_collection
            .insert_one(pocket, None)
            .await
            .ok()
            .expect("Error creating pocket"))
    }
}
