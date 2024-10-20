use diesel_async::{
    pooled_connection::deadpool::{Pool, PoolError},
    AsyncPgConnection,
};
use thiserror::Error;
use crate::graphql::user::CreateUserDTO;
use super::{
    model::{User},
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

    pub async fn create_user(&self, u: CreateUserDTO) -> Result<User, UserServiceError> {
        let mut conn = self.db_pool.get().await?;
        let user: User = u.into();
        repo::create_user(&mut conn, user.clone()).await?;
        Ok(user)
    }

    pub async fn get_user(&self,  user_id: uuid::Uuid) -> Result<User, UserServiceError> {
        let mut conn = self.db_pool.get().await?;
        let user = repo::get_user(&mut conn, user_id).await?;
        Ok(user)
    }
}
