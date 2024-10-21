use chrono::{DateTime, Utc};

use crate::{user, utils};
use crate::graphql::post::{CreatePostDTO, PostDTO};

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
