use actix_web::{get, HttpResponse, Responder};

#[get("/healthchecker")]
pub async fn healthchecker() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
