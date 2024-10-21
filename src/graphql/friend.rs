use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use crate::graphql::user::UserDTO;
use crate::user;

#[derive(SimpleObject)]
#[derive(Serialize)]
pub struct FriendsDTO {
    pub user: UserDTO,
    pub friends: Vec<UserDTO>,
}

#[derive(InputObject)]
#[derive(Deserialize)]
pub struct AddFriendDTO {
    pub user1: uuid::Uuid,
    pub user2: uuid::Uuid,
}
