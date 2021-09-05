use actix_web::{web, HttpResponse};
use mongodb::{bson::doc};
use serde::{Deserialize, Serialize};
use log::info;

#[derive(Deserialize, Serialize)]
pub struct User {
    username: String,
    password: String,
    email: String,
}

pub async fn get_users() -> HttpResponse {
    HttpResponse::Ok().body("get user not implemented yet")
}

pub async fn get_user_by_id() -> HttpResponse {
    HttpResponse::Ok().body("get user by id not implemented yet")
}

pub async fn add_user(user: web::Json<User>, data: web::Data<crate::AppState>) -> HttpResponse {
    // Get the User Collection Interface
    let user_service = &data.service_container.user;

    // Create the new user
    let new_user = doc! {
        "username": &user.username,
        "email": &user.email,
        // TODO: Encrypt the password
        "password": &user.password,
    };

    info!("inserted {}", new_user.get("username").unwrap());

    // Insert new user into database
    let _ = user_service.create(new_user.clone()).await.unwrap();

    // Respond with Created code and JSON of new user
    HttpResponse::Created().json(new_user)
}

pub async fn delete_user() -> HttpResponse {
    HttpResponse::Ok().body("delete user not implemented yet")
}
