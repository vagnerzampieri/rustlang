use std::env;
use dotenv::dotenv;

use futures::stream::TryStreamExt;

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

    // ----------------------------- TAG -----------------------------

    pub async fn get_tags(&self) -> Result<Vec<Tag>, Error> {
        let mut cursors = self.tag_collection
            .find(None, None)
            .await
            .ok()
            .expect("Error getting tags");

        let mut tags: Vec<Tag> = Vec::new();

        while let Some(tag) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            tags.push(tag);
        }

        Ok(tags)
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

    pub async fn create_tag(&self, tag: Tag) -> Result<InsertOneResult, Error> {
        Ok(self.tag_collection
            .insert_one(tag, None)
            .await
            .ok()
            .expect("Error creating tag"))
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

    // ----------------------------- POCKET -----------------------------

    pub async fn get_pockets(&self, tag_name: Option<&String>) -> Result<Vec<Pocket>, Error> {
        let filter = match tag_name {
            Some(tag_name) => doc! { "tags.name": tag_name },
            None => doc! {},
        };

        let mut cursors = self.pocket_collection
            .find(filter, None)
            .await
            .ok()
            .expect("Error getting pockets");

        let mut pockets: Vec<Pocket> = Vec::new();

        while let Some(pocket) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            pockets.push(pocket);
        }

        Ok(pockets)
    }

    pub async fn get_pocket(&self, id: &String) -> Result<Pocket, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };

        let pocket_detail = self.pocket_collection
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting pocket");

        Ok(pocket_detail.unwrap())
    }

    pub async fn create_pocket(&self, pocket: Pocket) -> Result<InsertOneResult, Error> {
        let new_doc = Pocket {
            id: None,
            created_at: chrono::Utc::now(),
            ..pocket
        };

        let new_pocket = self.pocket_collection
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating pocket");

        Ok(new_pocket)
    }

    pub async fn update_pocket(&self, id: &String, pocket: Pocket) -> Result<Pocket, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };

        Ok(self.pocket_collection
            .find_one_and_replace(filter, pocket, None)
            .await
            .ok()
            .expect("Error updating pocket")
            .unwrap())
    }

    pub async fn delete_pocket(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };

        let pocket = self.pocket_collection
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting pocket");

        Ok(pocket)
    }
}
