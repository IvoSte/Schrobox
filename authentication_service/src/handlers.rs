use actix_web::{web, HttpResponse};
use mongodb::{bson::doc};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use log::info;
use chrono::{UTC, Duration};
use std::collections::HashMap;


#[derive(Deserialize, Serialize)]
pub struct User {
    username: String,
    first_name: String,
    last_name: String,
    password: String,
    email: String,
}



#[derive(Deserialize)]
pub struct UserLogin {
    email: String,
    password: String,

}


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub async fn login(data: web::Data<crate::AppState>, user: web::Json<UserLogin>) -> HttpResponse {
    // TODO: Remove unwraps and handle them properly
    let user_service = &data.service_container.user;

    let query = doc! {
        "email": &user.email,
    };

    // Get the existing user from the database if it exists
    // If it does not exist, return an Internal Server Error
    let existing_user = user_service.get(query).await;
    let existing_user = match existing_user.unwrap() {
        Some(user_data) => user_data,
        // TODO: Some errors can be custom made, might be better, see https://actix.rs/docs/errors/
        None => {
            return HttpResponse::InternalServerError().body("User does not exist.");
        },
    };

    // Verify the password
    if !bcrypt::verify(&user.password, existing_user.get("password").unwrap().as_str().unwrap()).unwrap() {
        return HttpResponse::Unauthorized().body("Wrong Password");
    }

    // Generate JWT Token
    // TODO: Get JWT expiration from config file
    let expiration_time = UTC::now()
        .checked_add_signed(Duration::minutes(30))
        .expect("valid timestamp")
        .timestamp();

    let my_claims = Claims {
        sub: existing_user.get("username").unwrap().to_string(),
        company: "amigos".to_owned(),
        exp: expiration_time as usize,
    };

    // TODO: Add a refresh token
    // TODO: Generate a safe encryption key and get it from a config file
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("sks84fkls0vjJSk3#@jfD!kfdsvc".as_ref())).unwrap();


    // Create the payload that is to be returned
    let mut login_info = HashMap::new();

    login_info.insert(String::from("access_token"), token);
    login_info.insert(String::from("username"), existing_user.get("username").unwrap().to_string());

    HttpResponse::Ok().json(login_info)
}

pub async fn signup(data: web::Data<crate::AppState>, user: web::Json<User>) -> HttpResponse {
    // TODO: Remove unwraps and handle them properly
    // Get the User Collection Interface
    let user_service = &data.service_container.user;

    // Check if user already exists
    let query = doc! {
        "email": &user.email,
    };
    let existing_user = user_service.get(query).await;
    if let Some(_) = existing_user.unwrap() {
        return HttpResponse::InternalServerError().body("User already exists");
    };

    // Create the new user
    let new_user = doc! {
        "email": &user.email,
        "username": &user.username,
        "first_name": &user.first_name,
        "last_name": &user.last_name,
        "password": bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).unwrap(),
    };

    info!("inserted {} with password {}", new_user.get("username").unwrap(),new_user.get("password").unwrap());

    // Insert new user into database
    let _ = user_service.create(new_user.clone()).await.unwrap();

    // Respond with Created code and JSON of new user
    HttpResponse::Created().body("success")
}

pub async fn delete_user() -> HttpResponse {
    HttpResponse::Ok().body("delete user not implemented yet")
}

pub async fn get_user_by_id() -> HttpResponse {
    HttpResponse::Ok().body("get user by id not implemented yet")
}
