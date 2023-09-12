use std::collections::HashMap;

use crate::{
    models::pocket_model::Pocket,
    repository::mongodb_repo::MongoRepo
};

use crate::requests::pocket_request::PocketRequest;

use actix_web::{
    get, post, put, delete,
    web::{Data, Json, Path, Query},
    HttpResponse,
};

#[get("/pockets")]
pub async fn get_pockets(query: Query<HashMap<String, String>>, repo: Data<MongoRepo>) -> HttpResponse {
    let result = repo.get_pockets(query.get("tag")).await;

    match result {
        Ok(pockets) => HttpResponse::Ok().json(pockets),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/pockets/{id}")]
pub async fn get_pocket(path: Path<String>,repo: Data<MongoRepo>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("id is required");
    }

    let result = repo.get_pocket(&id).await;

    match result {
        Ok(pocket) => HttpResponse::Ok().json(pocket),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/pockets")]
// I'm working with the same Pocket I'm using on the model, so I need a specific
// to work with the Request body
pub async fn create_pocket(pocket: Json<PocketRequest>, repo: Data<MongoRepo>) -> HttpResponse {
    let result = repo.create_pocket(pocket.into_inner()).await;

    match result {
        Ok(pocket) => HttpResponse::Ok().json(pocket),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/pockets/{id}")]
pub async fn update_pocket(path: Path<String>, pocket: Json<Pocket>, repo: Data<MongoRepo>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("id is required");
    }

    let result = repo.update_pocket(&id, pocket.into_inner()).await;

    match result {
        Ok(_pocket) => {
            if let Some(updated_pocket) = repo.get_pocket(&id).await.ok() {
                HttpResponse::Ok().json(updated_pocket)
            } else {
                HttpResponse::InternalServerError().body("Error getting updated pocket")
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/pockets/{id}")]
pub async fn delete_pocket(path: Path<String>, repo: Data<MongoRepo>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("id is required");
    }

    let result = repo.delete_pocket(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                HttpResponse::Ok().body("Pocket deleted")
            } else {
                HttpResponse::InternalServerError().body("Error deleting pocket")
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
