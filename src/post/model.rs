use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{user, utils};

#[derive(Serialize)]
pub struct PostDTO {
    pub id: uuid::Uuid,
    pub posted_by: uuid::Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct PostsDTO {
    pub userdto: user::model::UserDTO,
    pub post_dtos: Vec<PostDTO>,
}

impl PostsDTO {
    pub fn new(userdto: user::model::UserDTO, post_dtos: Vec<PostDTO>) -> PostsDTO {
        PostsDTO { userdto, post_dtos }
    }
}

#[derive(Deserialize)]
pub struct CreatePostDTO {
    #[serde(skip_deserializing)]
    pub posted_by: uuid::Uuid,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub id: uuid::Uuid,
    pub posted_by: uuid::Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<CreatePostDTO> for Post {
    fn from(p: CreatePostDTO) -> Self {
        Post {
            id: uuid::Uuid::new_v4(),
            posted_by: p.posted_by,
            content: p.content,
            created_at: utils::time_now(),
        }
    }
}

impl Into<PostDTO> for Post {
    fn into(self) -> PostDTO {
        PostDTO {
            id: self.id,
            posted_by: self.posted_by,
            content: self.content,
            created_at: self.created_at,
        }
    }
}
