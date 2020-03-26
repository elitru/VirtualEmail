use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    email: String,
    password: String,
    created_on: chrono::NaiveDateTime
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

#[derive(Serialize, Deserialize)]
pub struct Message{
    id: Uuid,
    sender_email: String,
    receiver_email: String,
    title: String,
    body: String,
    sent_on: chrono::NaiveDateTime
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