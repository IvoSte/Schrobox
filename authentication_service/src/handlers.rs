use actix_web::{web, HttpResponse};
use mongodb::{bson::doc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::jwt;

use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};

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


pub async fn login(data: web::Data<crate::AppState>, user: web::Json<UserLogin>) -> HttpResponse {
    // TODO: Remove unwraps and handle them properly
    let user_service = &data.service_container.user;


    // Get the existing user from the database if it exists
    // If it does not exist, return an Internal Server Error
    let query = doc! {
        "email": &user.email,
    };

    let existing_user = user_service.get(query).await;
    let existing_user = match existing_user.unwrap() {
        Some(user_data) => user_data,
        // TODO: Some errors can be custom made, might be better, see https://actix.rs/docs/errors/
        None => {
            return HttpResponse::InternalServerError().body("User does not exist.");
        },
    };

    // Verify the password
    // if !bcrypt::verify(&user.password, existing_user.get("password").unwrap().as_str().unwrap()).unwrap() {
    let parsed_hash = PasswordHash::new(existing_user.get("password").unwrap().as_str().unwrap()).unwrap();
    if !Pbkdf2.verify_password(&user.password.as_bytes(), &parsed_hash).is_ok() {
        return HttpResponse::Unauthorized().body("Wrong Password");
    }

    // Generate JWT token
    let token = jwt::generate_jwt_token(existing_user.get("username").unwrap().to_string()).unwrap();


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
    if existing_user.unwrap().is_some() {
        return HttpResponse::InternalServerError().body("User already exists");
    };

    // Hash password with salt to PHC string ($pbkdf2-sha256$...)
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Pbkdf2.hash_password(&user.password.as_bytes(), &salt).unwrap().to_string();

    // Create the new user
    let new_user = doc! {
        "email": &user.email,
        "username": &user.username,
        "first_name": &user.first_name,
        "last_name": &user.last_name,
        "password": password_hash,
    };

    // Insert new user into database
    let _ = user_service.create(new_user.clone()).await.unwrap();

    // Respond with Created code
    HttpResponse::Created().body("success")
}

pub async fn delete_user() -> HttpResponse {
    HttpResponse::Ok().body("delete user not implemented yet")
}

pub async fn get_user_by_id() -> HttpResponse {
    HttpResponse::Ok().body("get user by id not implemented yet")
}
