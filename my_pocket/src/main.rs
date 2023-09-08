mod api;
mod models;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use env_logger;
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

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .service(healthchecker)
            .service(create_tag)
            .service(create_pocket)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
