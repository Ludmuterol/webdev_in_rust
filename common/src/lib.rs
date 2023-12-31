use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

impl LoginData {
    pub fn to_str(self: &Self) -> Option<String> {
        match serde_json::to_string(self) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
    pub fn from_str(str: &String) -> Option<LoginData> {
        match serde_json::from_str(str) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ProfileData {
    pub username: String,
}

impl ProfileData {
    pub fn to_str(self: &Self) -> Option<String> {
        match serde_json::to_string(self) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
    pub fn from_str(str: &String) -> Option<ProfileData> {
        match serde_json::from_str(str) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ReviewData {
    pub website: String,
    pub comment: String,
    pub username: String,
    pub score: String,
}
impl ReviewData {
    pub fn to_str(self: &Self) -> Option<String> {
        match serde_json::to_string(self) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
    pub fn from_str(str: &String) -> Option<ReviewData> {
        match serde_json::from_str(str) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}
