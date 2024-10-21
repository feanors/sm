use async_graphql::InputObject;
use serde::Deserialize;

#[derive(InputObject)]
#[derive(Deserialize)]
pub struct CreateLikeDTO {
    pub liked_by: uuid::Uuid,
    pub liked_post: uuid::Uuid,
}
