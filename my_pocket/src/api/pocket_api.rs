use crate::{
    models::pocket_model::Pocket,
    repository::mongodb_repo::MongoRepo
};

use actix_web::{
    post,
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
