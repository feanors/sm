use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateLikeDTO {
    pub liked_by: uuid::Uuid,
    #[serde(skip_deserializing)]
    pub liked_post: uuid::Uuid,
}

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
