use actix_web::{http::StatusCode, Error, error, HttpRequest, HttpResponse, dev::Payload, FromRequest};
use actix_http::{http::header, ResponseBuilder};
use futures::future;
use serde::{Serialize, Deserialize};
use derive_more::{Display, Error};

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

#[derive(Debug, Display, Error)]
enum AuthError {
    #[display(fmt = "unauthorized")]
    UnauthorizedError,

    #[display(fmt = "bad request")]
    BadRequestError,
}

impl error::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AuthError::UnauthorizedError => StatusCode::UNAUTHORIZED,
            AuthError::BadRequestError => StatusCode::BAD_REQUEST,
        }
    }
}

pub struct User {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl FromRequest for User {
    type Error = Error;
    type Future = future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Some(token) = req.headers().get("Authorization") {
            match verify_jwt(&token.to_str().unwrap()) {
                Ok(token_data) => {

                    let user = User {
                        username: token_data.sub
                    };

                    future::ready(Ok(user))
                },
                Err(e) => future::ready(Err(e)),
            }
        } else {
            future::ready(Err(Error::from(AuthError::BadRequestError)))
        }
    }
}

fn verify_jwt(token: &str) -> Result<Claims, Error> {
    // token.split()
    let token_message = decode::<Claims>(
        &token,
        // TODO: Get secret from config and randomly generate it
        &DecodingKey::from_secret("sks84fkls0vjJSk3#@jfD!kfdsvc".as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    println!("{}", token);
    match token_message {
        Ok(token_data) => Ok(token_data.claims),
        Err(error) => Err(Error::from(AuthError::UnauthorizedError)),
    }
}
