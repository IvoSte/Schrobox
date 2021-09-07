use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse};
use env_logger::Builder;
use log::LevelFilter;

mod auth;

pub async fn filestorage(authed_user: auth::User) -> HttpResponse {
    HttpResponse::Ok().body(format!("hello {}!", authed_user.username))
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
        App::new()
            .wrap(Logger::default())
            .route("/filestorage", web::post().to(filestorage))
    })
    .bind("127.0.0.1:9002")?
    .run()
    .await
}
