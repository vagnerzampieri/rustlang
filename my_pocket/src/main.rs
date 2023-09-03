use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/pockets")]
async fn pockets() -> impl Responder {
    HttpResponse::Ok().body("List of links")
}

#[post("/create_pocket")]
async fn create_pocket(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/pocket/{id}")]
async fn pocket_by_id(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Pocket with id: {}", id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(pockets)
            .service(create_pocket)
            .service(pocket_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
