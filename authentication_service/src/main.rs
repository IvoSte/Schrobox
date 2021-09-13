use actix_web::{middleware::Logger, web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use user_api::UserService;
use env_logger::Builder;
use log::LevelFilter;
use log::info;
use std::env;
use dotenv;

mod handlers;
mod jwt;

pub struct AppState {
    user_service: UserService,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    info!("Starting authentication service...");
    // Load env file
    dotenv::dotenv().ok();

    // Logger related
    std::env::set_var("RUST_LOG", "actix_web=debug");
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    // Parse your connection string into an options struct
    let client_options =
        ClientOptions::parse(format!("mongodb://{}:{}@{}:{}", env::var("MONGODB_USER").unwrap(), env::var("MONGODB_PASS").unwrap(), env::var("MONGODB_HOST").unwrap(), env::var("MONGODB_PORT").unwrap()).as_ref())
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(&env::var("MONGODB_DB_NAME").unwrap());
    let user_collection = db.collection(&env::var("MONGODB_USER_COLLECTION").unwrap());

    // Start http server
    HttpServer::new(move || {

        let user_service = UserService::new(user_collection.clone());
        App::new()
            .data(AppState {
                user_service,
            })
            .wrap(Logger::default())
            .route("/login", web::post().to(handlers::login))
            .route("/signup", web::post().to(handlers::signup))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind(format!("{}:{}", env::var("AUTH_HOST").unwrap(), env::var("AUTH_PORT").unwrap()))?
    .run()
    .await
}
