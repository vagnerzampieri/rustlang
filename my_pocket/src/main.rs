use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;
use mongodb::{Client, Database, bson, bson::doc, bson::oid::ObjectId, error::Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pocket {
    #[serde(rename = "_id")]
    id: ObjectId,
    description: String,
    link: String,
    tags: Vec<Tag>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[get("/healthchecker")]
async fn healthchecker() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/pockets")]
async fn pockets() -> impl Responder {
    let db = db().await.unwrap();
    let collection = db.collection("pockets");
    let mut cursor = collection.find(doc! {}, None).await.unwrap();
    let mut pockets: Vec<Pocket> = Vec::new();

    for result in cursor {
        if let Ok(document) = result {
            let pocket: Pocket = bson::from_bson(bson::Bson::Document(document)).unwrap();
            pockets.push(pocket);
        }
    }

    HttpResponse::Ok().json(pockets)
}

#[post("/create_pocket")]
async fn create_pocket(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/pocket/{id}")]
async fn pocket_by_id(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Pocket with id: {}", id))
}

async fn db() -> Result<Database> {
    let client = Client::with_uri_str(&env::var("MONGO_URI").unwrap()).await.unwrap();
    let db: Database = client.database(&env::var("MONGO_DB").unwrap());
    Ok(db)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(healthchecker)
            .service(pockets)
            .service(create_pocket)
            .service(pocket_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
