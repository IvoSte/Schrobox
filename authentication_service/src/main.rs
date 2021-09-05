use actix_web::{middleware::Logger, web, App, HttpServer};
use mongodb::{bson::doc, options::ClientOptions, Client};
use users::UserService;
use env_logger::Builder;
use log::LevelFilter;


mod handlers;
mod users;

pub struct ServiceContainer {
    user: UserService,
}

impl ServiceContainer {
    pub fn new(user: UserService) -> Self {
        ServiceContainer { user }
    }
}

pub struct AppState {
    service_container: ServiceContainer,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Logger related
    std::env::set_var("RUST_LOG", "actix_web=debug");
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    // Parse your connection string into an options struct
    let client_options =
        ClientOptions::parse("mongodb://main_admin:abc123@0.0.0.0:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();

    // Check if the mongo connection is successfull
    let res = client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await
        .unwrap();
    println!("{}", res);
    println!("Connected successfully.");

    let db = client.database("schro-database");
    let user_collection = db.collection("users");

    // Start http server
    HttpServer::new(move || {

        let service_container = ServiceContainer::new(UserService::new(user_collection.clone()));
        App::new()
            .data(AppState {
                service_container,
            })
            .wrap(Logger::default())
            .route("/login", web::post().to(handlers::login))
            .route("/signup", web::post().to(handlers::signup))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
