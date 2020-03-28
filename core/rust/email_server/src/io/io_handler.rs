use super::super::errors::APIError;
use crate::models::models::{Message, User};
use async_std::fs;
use uuid::Uuid;

const USER_PATH: &'static str = "../resources/user_data.json";

pub async fn get_user(email: &str) -> Result<Option<User>, APIError> {
    let users: Vec<User> = get_all_users().await?;
    
    for user in users.into_iter() {
        if user.email == email {
            return Ok(Some(user));
        }
    }

    Ok(None)
}

pub async fn get_message(id: &Uuid) -> Message {
    todo!()
}

pub async fn save_messages(message: &Message) {
    todo!()
}

async fn get_all_users() -> Result<Vec<User>, APIError> {
    let content: String = fs::read_to_string(USER_PATH).await?;
    let users: Vec<User> = serde_json::from_str(&content[..])?;
    return Ok(users);
}

async fn save_all_users(users: &Vec<User>) -> Result<(), APIError> {
    let as_json:String = serde_json::to_string(users)?;
    fs::write(USER_PATH, &as_json).await?;
    Ok(())
}