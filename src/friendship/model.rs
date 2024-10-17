use serde::{Deserialize, Serialize};

use crate::user;

#[derive(Serialize)]
pub struct FriendsDTO {
    pub user: user::model::UserDTO,
    pub friends: Vec<user::model::UserDTO>,
}

#[derive(Deserialize)]
pub struct AddFriendDTO {
    #[serde(skip_deserializing)]
    pub user1: uuid::Uuid,
    pub user2: uuid::Uuid,
}
