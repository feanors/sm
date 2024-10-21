use chrono::{DateTime, Utc};

use crate::graphql::user::{CreateUserDTO, UserDTO};
use crate::utils;



#[derive(Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl Into<UserDTO> for User {
    fn into(self) -> UserDTO {
        UserDTO {
            id: self.id,
            username: self.username,
            description: self.description,
            created_at: self.created_at,
        }
    }
}

impl From<CreateUserDTO> for User {
    fn from(u: CreateUserDTO) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            username: u.username,
            description: u.description,
            created_at: utils::time_now(),
        }
    }
}
