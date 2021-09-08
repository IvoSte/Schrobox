use jsonwebtoken::{encode, Header, EncodingKey, errors::Error};
use chrono::{UTC, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn generate_jwt_token(username: String) -> Result<String, Error> {
    // Generate JWT Token
    // TODO: Get JWT expiration from config file
    let expiration_time = UTC::now()
        .checked_add_signed(Duration::minutes(30))
        .expect("valid timestamp")
        .timestamp();

    let my_claims = Claims {
        sub: username.to_string(),
        company: "amigos".to_owned(),
        exp: expiration_time as usize,
    };

    // TODO: Add a refresh token
    // TODO: Generate a safe encryption key and get it from a config file
    encode(&Header::default(), &my_claims, &EncodingKey::from_secret("sks84fkls0vjJSk3#@jfD!kfdsvc".as_ref()))
}
