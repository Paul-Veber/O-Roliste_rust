use std::time::SystemTime;

use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub pseudo: String,
    pub email: String,
    pub password: String,
    pub age: Option<i32>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}