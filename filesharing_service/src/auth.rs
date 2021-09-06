use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::AuthenticationError;
use log::info;

fn validate_token(str: &str) -> Result<bool, std::io::Error>
{
    println!("validating token...");
    if str.eq("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJcImpvaG5kb2U0M1wiIiwiY29tcGFueSI6ImFtaWdvcyIsImV4cCI6MTYzMDk0MzM0MX0.Tx4M3DsLs_mp_SykbmpK7RK6xaC418BEJWYN_i5nDu4")
    {
        println!("Validating token to be Okay");
        return Ok(true);
    }
    println!("Token Invalid!");
    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Authentication failed!"));
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    println!("starting validation");
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    info!("{}", credentials.token());
    match validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
