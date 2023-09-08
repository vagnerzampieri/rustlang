use crate::{
  models::tag_model::Tag,
  repository::mongodb_repo::MongoRepo
};

use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};

#[post("/tags")]
pub async fn create_tag(tag: Json<Tag>, repo: Data<MongoRepo>) -> HttpResponse {
  let result = repo.create_tag(tag.into_inner()).await;

  match result {
    Ok(tag) => HttpResponse::Ok().json(tag),
    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
}
