use uuid::Uuid;
use crate::models::models::{User, Message};
use async_std::fs;
use super::super::errors::APIError;

pub async fn get_user(email: &str) -> User {
    todo!()
}

pub async fn save_user(user: &User) {
    todo!()
}

pub async fn get_message(id: &Uuid) -> Message {
    todo!()
}

pub async fn save_message(message: &Message) {
    todo!()
}

async fn get_all_users() -> Result<Vec<User>, APIError> {
    let content: String = fs::read_to_string("../resources/user_data.json").await?;
    let users: Vec<User> = serde_json::from_str(&content[..])?;
    return Ok(users);
}