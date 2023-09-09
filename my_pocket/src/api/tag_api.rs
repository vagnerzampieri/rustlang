use crate::{
    models::tag_model::Tag,
    repository::mongodb_repo::MongoRepo
};

use actix_web::{
    post, get, put,
    web::{Data, Json, Path},
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

#[get("/tags/{id}")]
pub async fn get_tag(path: Path<String>, repo: Data<MongoRepo>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("id is required");
    }

    let result = repo.get_tag(&id).await;

    match result {
        Ok(tag) => HttpResponse::Ok().json(tag),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/tags/{id}")]
pub async fn update_tag(path: Path<String>, tag: Json<Tag>, repo: Data<MongoRepo>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("id is required");
    }

    let result = repo.update_tag(&id, tag.into_inner()).await;

    match result {
        Ok(_tag) => {
            if let Some(updated_tag) = repo.get_tag(&id).await.ok() {
                return HttpResponse::Ok().json(updated_tag);
            } else {
                return HttpResponse::InternalServerError().body("Error getting updated tag");
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
