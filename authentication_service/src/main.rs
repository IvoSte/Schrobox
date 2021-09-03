
use mongodb::bson::{self, doc, Bson};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, get, guard, middleware::Logger, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Result,
};

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let client_uri = "mongodb://main_admin:abc123@0.0.0.0:27017";
    // A Client is needed to connect to MongoDB:

    // An extra line of code to work around a DNS issue on Windows:

    // let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;

    // let client = mongodb::Client::with_options(options);
    // println!("Starting Authentication Service...");

    // println!("Databases:");
    // for name in client.list_database_names(None, None).await {
    //     println!("- {}", name);

    // }


    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/login", web::post().to(handlers::get_users))
            .route("/signup", web::post().to(handlers::add_user))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
