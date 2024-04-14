use chrono::{Duration, Local};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::Algorithm::HS256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
    username: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtResponse{
    pub(crate) access_token: String,
    pub(crate) refresh_token: String
}

pub async fn create_access_token(user: &String) -> Result<String, jsonwebtoken::errors::Error> {

    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(1)).timestamp();
    let claims = Claims {
        iss: user.to_string(),
        sub: String::from("API Rust"),
        iat,
        exp,
        username: user.to_string()
    };

    let token = encode(&Header::new(HS256), &claims, &EncodingKey::from_secret("superSecretKey".as_ref()));

    match token {
        Ok(value) => { return Ok(value)}
        Err(e) => {return Err(e) }
    }
}

pub async fn create_refresh_token(user: &String) -> Result<String, jsonwebtoken::errors::Error> {

    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + Duration::weeks(1)).timestamp();
    let claims = Claims {
        iss: user.to_string(),
        sub: String::from("API Rust"),
        iat,
        exp,
        username: user.to_string()

    };

    let token = encode(&Header::new(HS256), &claims, &EncodingKey::from_secret("superSecretKey".as_ref()));

    match token {
        Ok(value) => { return Ok(value)}
        Err(e) => {return Err(e) }
    }}

fn validate_token(token: &str) -> Result<(), String> {
    let decoding_key = DecodingKey::from_secret("superSecretKey".as_ref());

    let validation = Validation::new(HS256);

    match decode::<serde_json::Value>(token, &decoding_key, &validation) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Token validation failed: {}", e)),
    }
}

