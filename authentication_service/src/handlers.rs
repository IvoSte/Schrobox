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

#[derive(Deserialize)]
pub struct UserLogin {
    username: String,
    password: String,
}

pub async fn login(data: web::Data<crate::AppState>, user: web::Json<UserLogin>) -> HttpResponse {
    // TODO: Remove unwraps and handle them properly
    let user_service = &data.service_container.user;

    let query = doc! {
        "username": &user.username,
    };

    let existing_user = user_service.get(query).await.unwrap().unwrap();

    if existing_user.get("password").unwrap().as_str().unwrap() != user.password {
        return HttpResponse::Unauthorized().body("Wrong Password");
    }


    // TODO: Return JWT token

    HttpResponse::Ok().body("login successfull")
}

pub async fn get_user_by_id() -> HttpResponse {
    HttpResponse::Ok().body("get user by id not implemented yet")
}

pub async fn signup(data: web::Data<crate::AppState>, user: web::Json<User>) -> HttpResponse {
    // TODO: Remove unwraps and handle them properly
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
