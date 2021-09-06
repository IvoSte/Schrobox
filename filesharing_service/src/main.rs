use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse};
use env_logger::Builder;
use log::LevelFilter;

mod auth;

pub async fn filestorage() -> HttpResponse {
    HttpResponse::Ok().body("You have succesfully send us a file")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Logger related
    std::env::set_var("RUST_LOG", "actix_web=debug");
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();

    // Start http server
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(auth::validator);

        App::new()
            .wrap(auth)
            .wrap(Logger::default())
            .route("/filestorage", web::post().to(filestorage))
    })
    .bind("127.0.0.1:9002")?
    .run()
    .await
}
