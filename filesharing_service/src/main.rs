use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use dotenv;
use env_logger::Builder;
use log::{info, LevelFilter};
use serde::{Serialize, Deserialize};
use std::fs;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;

mod auth;

#[derive(Deserialize)]
pub struct Download {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    time: u64,
    err: String
}

const UPLOAD_PATH: &str = "./files/upload";

async fn upload(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    // iterate over multipart stream
    fs::create_dir_all(UPLOAD_PATH)?;
    let mut filename = "".to_string();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        filename = format!("{} - {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(), content_type.get_filename().unwrap(), );
        let filepath = format!("{}/{}", UPLOAD_PATH, sanitize_filename::sanitize(&filename));
        // File::create is blocking operation, use thread pool
        let mut f = web::block(|| fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use thread pool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    // Create a unique name for the file
    let res = &File {
        name: filename,
        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),err: "".to_string()    };
    Ok(HttpResponse::Ok().json(res))
}

async fn download(info: web::Path<Download>) -> HttpResponse {
    let path = format!("{}/{}", UPLOAD_PATH, info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&File {
            name: info.name.to_string(),
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            err: "file does not exists".to_string(),
        });
    }
    let data = fs::read(path).unwrap();
    HttpResponse::Ok()
        .header("Content-Disposition", format!("form-data; filename={}", info.name.to_string()))
        .body(data)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Logger related
    std::env::set_var("RUST_LOG", "actix_web=debug");
    Builder::new().filter(None, LevelFilter::Debug).init();

    info!("Starting filesharing service...");
    // Load env file
    dotenv::dotenv().ok();

    // Start http server
    HttpServer::new(move || {
        App::new().wrap(Logger::default()).service(
            web::scope("/api")
                .route("/files/", web::post().to(upload))
                .route("/files/{name}/", web::get().to(download)),
        )
    })
    .bind("127.0.0.1:9002")?
    .run()
    .await
}
