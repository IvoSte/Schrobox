use actix_web::{web, HttpResponse};
use mongodb::{bson::doc};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use log::info;
use chrono::{UTC, Duration};

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
        "username": &user.username,
    };

    let existing_user = user_service.get(query).await.unwrap().unwrap();
    // TODO: Check for non-existing users

    println!("{} == {}", user.password, existing_user.get("password").unwrap().as_str().unwrap());

    if !bcrypt::verify(&user.password, existing_user.get("password").unwrap().as_str().unwrap()).unwrap() {
        return HttpResponse::Unauthorized().body("Wrong Password");
    }


    // Generate JWT Token
    let expiration_time = UTC::now()
        .checked_add_signed(Duration::minutes(30))
        .expect("valid timestamp")
        .timestamp();

    let my_claims = Claims {
        sub: user.username.to_owned(),
        company: "amigos".to_owned(),
        // TODO: Get JWT expiration from config file
        exp: expiration_time as usize,
    };

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("sks84fkls0vjJSk3#@jfD!kfdsvc".as_ref())).unwrap();

    HttpResponse::Ok().json(token)
}

pub async fn get_user_by_id() -> HttpResponse {
    HttpResponse::Ok().body("get user by id not implemented yet")
}

pub async fn signup(data: web::Data<crate::AppState>, user: web::Json<User>) -> HttpResponse {
    // TODO: Remove unwraps and handle them properly
    // Get the User Collection Interface
    let user_service = &data.service_container.user;

    // TODO: Check for duplicates
    // Create the new user
    let new_user = doc! {
        "username": &user.username,
        "email": &user.email,
        "password": bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).unwrap(),
    };

    info!("inserted {} with password {}", new_user.get("username").unwrap(),new_user.get("password").unwrap());

    // Insert new user into database
    let _ = user_service.create(new_user.clone()).await.unwrap();

    // Respond with Created code and JSON of new user
    HttpResponse::Created().json(new_user)
}

pub async fn delete_user() -> HttpResponse {
    HttpResponse::Ok().body("delete user not implemented yet")
}
