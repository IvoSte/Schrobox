use jsonwebtoken::{encode, Header, EncodingKey, errors::Error};
use chrono::{UTC, Duration};
use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    static ref SECRET_KEY: String = std::env::var("JWT_SECRET_KEY").unwrap();
    static ref CLAIM_ISSUER: String = std::env::var("JWT_ISSUER").unwrap();
    static ref EXPIRATION_TIME: i64 = std::env::var("JWT_EXPIRE_TIME").unwrap().parse::<i64>().unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iss: String,
    iat: usize,
}

pub fn generate_jwt_token(username: String) -> Result<String, Error> {
    // Generate JWT Token
    let expiration_time = UTC::now()
        .checked_add_signed(Duration::minutes(*EXPIRATION_TIME))
        .expect("valid timestamp")
        .timestamp();

    let my_claims = Claims {
        sub: username.to_string(),
        exp: expiration_time as usize,
        iss: CLAIM_ISSUER.to_string(),
        iat: UTC::now().timestamp() as usize,
    };

    encode(&Header::default(), &my_claims, &EncodingKey::from_secret(SECRET_KEY.as_bytes()))
}

// TODO: Add a refresh token
pub fn generate_jwt_refresh_token(username: String, jwt_token_collection: mongodb::Collection) {
    // Generate JWT Refresh Token
    let expiration_time = UTC::now()
        .checked_add_signed(Duration::minutes(*EXPIRATION_TIME))
        .expect("valid timestamp")
        .timestamp();

    let my_claims = Claims {
        sub: username.to_string(),
        exp: expiration_time as usize,
        iss: CLAIM_ISSUER.to_string(),
        iat: UTC::now().timestamp() as usize,
    };

    // encode(&Header::default(), &my_claims, &EncodingKey::from_secret(SECRET_KEY.as_bytes()))
}
