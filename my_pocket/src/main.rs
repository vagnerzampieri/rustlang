mod api;
mod models;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use env_logger;
use std::env;
use repository::mongodb_repo::MongoRepo;

use api::{
    tag_api::{get_tag, get_tags, create_tag, update_tag, delete_tag},
    pocket_api::{create_pocket, get_pockets},
    health_checker_api::healthchecker,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .service(healthchecker)
            .service(create_tag)
            .service(get_tag)
            .service(update_tag)
            .service(delete_tag)
            .service(get_tags)
            .service(create_pocket)
            .service(get_pockets)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
