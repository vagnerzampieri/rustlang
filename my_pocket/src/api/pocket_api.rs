use crate::{
    models::pocket_model::Pocket,
    repository::mongodb_repo::MongoRepo
};

use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/pockets")]
pub async fn create_pocket(pocket: Json<Pocket>, repo: Data<MongoRepo>) -> HttpResponse {
    let result = repo.create_pocket(pocket.into_inner()).await;

    match result {
        Ok(pocket) => HttpResponse::Ok().json(pocket),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/pockets")]
pub async fn get_pockets(repo: Data<MongoRepo>) -> HttpResponse {
    let result = repo.get_pockets().await;

    match result {
        Ok(pockets) => HttpResponse::Ok().json(pockets),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
