use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq,)]
pub struct User {
    pub email: String,
    pub password: String,
    pub created_on: chrono::NaiveDateTime
}

impl User{
    pub fn new(email: &str, password: &str) -> Self{
        Self{
            email: email.to_owned(),
            password: password.to_owned(),
            created_on: chrono::Local::now().naive_local() //current time
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq,)]
pub struct Message{
    pub id: Uuid,
    pub sender_email: String,
    pub receiver_email: String,
    pub title: String,
    pub body: String,
    pub sent_on: chrono::NaiveDateTime
}

impl Message{
    pub fn new(id: Uuid, sender_email: &str, receiver_email: &str, title: &str, body: &str) -> Self{
        Self{
            id,
            sender_email: sender_email.to_owned(),
            receiver_email: receiver_email.to_owned(),
            title: title.to_owned(),
            body: body.to_owned(),
            sent_on: chrono::Local::now().naive_local(),
        }
    }
}