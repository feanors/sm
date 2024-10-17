use diesel_async::{
    pooled_connection::deadpool::{Pool, PoolError},
    AsyncPgConnection,
};
use thiserror::Error;

use super::{
    model::{CreateUserDTO, User, UserDTO},
    repo,
};

pub struct UserService {
    db_pool: Pool<AsyncPgConnection>,
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("database error")]
    DBResult(#[from] diesel::result::Error),
    #[error("database driver error")]
    DBPool(#[from] PoolError),
}

impl UserService {
    pub fn new(db_pool: Pool<AsyncPgConnection>) -> UserService {
        UserService { db_pool }
    }

    pub async fn create_user(&self, u: CreateUserDTO) -> Result<UserDTO, UserServiceError> {
        let mut conn = self.db_pool.get().await?;
        let user = User::from_userdto(u);
        repo::create_user(&mut conn, &user).await?;
        Ok(user.to_userdto())
    }

    pub async fn get_user(&self, user_id: uuid::Uuid) -> Result<UserDTO, UserServiceError> {
        let mut conn = self.db_pool.get().await?;
        let user = repo::get_user(&mut conn, user_id).await?;
        Ok(user.to_userdto())
    }
}
