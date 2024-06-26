use actix_web::guard::GuardContext;
use actix_web::{HttpRequest, HttpResponse};
use chrono::{Duration, Local};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::Algorithm::HS256;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

const SECRETKEY: &str = "superSecretKey";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtResponse {
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
}

pub async fn create_access_token(user: &String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(1)).timestamp();
    let claims = Claims {
        iss: String::from("API Rust"),
        sub: user.to_string(),
        iat,
        exp,
    };

    let token = encode(
        &Header::new(HS256),
        &claims,
        &EncodingKey::from_secret(SECRETKEY.as_ref()),
    );

    match token {
        Ok(value) => return Ok(value),
        Err(e) => return Err(e),
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
    };

    let token = encode(
        &Header::new(HS256),
        &claims,
        &EncodingKey::from_secret(SECRETKEY.as_ref()),
    );

    match token {
        Ok(value) => return Ok(value),
        Err(e) => return Err(e),
    }
}

fn validate_token(token: &str) -> Result<(), String> {
    let decoding_key = DecodingKey::from_secret(SECRETKEY.as_ref());

    let validation = Validation::new(HS256);
    let binding = token.replace("Bearer ", "");
    let token_value = binding.as_str();

    match decode::<serde_json::Value>(token_value, &decoding_key, &validation) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Token validation failed: {}", e)),
    }
}

pub fn get_claims(req: &HttpRequest) -> Result<Claims, jsonwebtoken::errors::ErrorKind> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                let decoding_key = DecodingKey::from_secret(SECRETKEY.as_ref());
                let validation = Validation::new(HS256);

                return match decode::<Claims>(token, &decoding_key, &validation) {
                    Ok(token_data) => {
                         Ok(token_data.claims)
                    }
                    Err(e) => {
                         Err(jsonwebtoken::errors::ErrorKind::InvalidToken)
                    }
                }
            }
        }
    }
    Err(jsonwebtoken::errors::ErrorKind::InvalidToken)
}

pub fn verify_token(ctx: &GuardContext) -> bool {
    let auth_header = ctx.head().headers().get("authorization");
    if auth_header.is_none() {
        HttpResponse::Unauthorized().json(json!({"error" : "Invalid token"}));
        return false;
    }
    let token = auth_header.unwrap().to_str().unwrap();
    validate_token(token).is_ok()
}

pub async fn handle_unauthorized() -> HttpResponse {
    HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}))
}