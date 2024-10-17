use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Serialize)]
pub struct UserDTO {
    pub id: uuid::Uuid,
    pub username: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub description: String,
}

#[derive(Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn to_userdto(self) -> UserDTO {
        UserDTO {
            id: self.id,
            username: self.username,
            description: self.description,
            created_at: self.created_at,
        }
    }

    pub fn from_userdto(u: CreateUserDTO) -> User {
        User {
            id: uuid::Uuid::new_v4(),
            username: u.username,
            description: u.description,
            created_at: utils::time_now(),
        }
    }
}
