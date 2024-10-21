use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use crate::graphql::like::CreateLikeDTO;

#[derive(Deserialize, Serialize, Debug)]
pub struct LikeEvent {
    pub liked_by: uuid::Uuid,
    pub liked_post: uuid::Uuid,
}

impl LikeEvent {
    pub fn new(c: &CreateLikeDTO) -> LikeEvent {
        LikeEvent {
            liked_by: c.liked_by,
            liked_post: c.liked_post,
        }
    }
}
