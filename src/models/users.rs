use std::ffi::CString;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]

pub struct User {
    pub username: String,
    pub password: String,
    pub refresh_token: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser{
    pub username: String,
    pub password: String,
}