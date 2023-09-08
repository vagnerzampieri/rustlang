mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use repository::mongodb_repo::MongoRepo;

use api::{
    tag_api::create_tag,
    pocket_api::create_pocket,
    health_checker_api::healthchecker,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(healthchecker)
            .service(create_tag)
            .service(create_pocket)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
