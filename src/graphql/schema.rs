use async_graphql::{Context, EmptySubscription, Object, Schema};
use std::sync::Arc;
use thiserror::Error;

use crate::friendship::service::{FriendshipService, FriendshipServiceError};
use crate::graphql::friend::AddFriendDTO;
use crate::graphql::like::CreateLikeDTO;
use crate::graphql::post::{CreatePostDTO, PostDTO};
use crate::graphql::user::{CreateUserDTO, UserDTO};
use crate::like::service::{LikeService, LikeServiceError};
use crate::post::service::{PostService, PostServiceError};
use crate::user::service::{UserService, UserServiceError};

pub struct Query {
    user_service: Arc<UserService>,
    post_service: Arc<PostService>,
    friendship_service: Arc<FriendshipService>,
    like_service: Arc<LikeService>,
}

pub struct Mutation {
    user_service: Arc<UserService>,
    post_service: Arc<PostService>,
    friendship_service: Arc<FriendshipService>,
    like_service: Arc<LikeService>,
}

#[derive(Error, Debug)]
pub enum GQLError {
    #[error("user service error")]
    UserService(#[from] UserServiceError),
    #[error("post service error")]
    PostService(#[from] PostServiceError),
    #[error("user service error")]
    LikeService(#[from] LikeServiceError),
    #[error("friendship service error")]
    FriendshipService(#[from] FriendshipServiceError),
}

#[Object]
impl Query {
    async fn get_user(&self, ctx: &Context<'_>, user_id: uuid::Uuid) -> Result<UserDTO, GQLError> {
        let user: UserDTO = self.user_service.get_user(user_id).await?.into();
        Ok(user)
    }

    async fn get_posts(
        &self,
        ctx: &Context<'_>,
        user_id: uuid::Uuid,
    ) -> Result<Vec<PostDTO>, GQLError> {
        let posts: Vec<PostDTO> = self
            .post_service
            .get_posts(user_id)
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect();
        Ok(posts)
    }

    async fn get_friends(
        &self,
        ctx: &Context<'_>,
        user_id: uuid::Uuid,
    ) -> Result<Vec<UserDTO>, GQLError> {
        let friends = self
            .friendship_service
            .get_friends(user_id)
            .await?
            .into_iter()
            .map(|f| f.into())
            .collect();
        Ok(friends)
    }

    async fn get_likes(
        &self,
        ctx: &Context<'_>,
        post_id: uuid::Uuid,
    ) -> Result<Vec<UserDTO>, GQLError> {
        let likes = self
            .like_service
            .get_likes(post_id)
            .await?
            .into_iter()
            .map(|l| l.into())
            .collect();
        Ok(likes)
    }
}

#[Object]
impl Mutation {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserDTO,
    ) -> Result<UserDTO, GQLError> {
        let res = self.user_service.create_user(input).await?.into();
        Ok(res)
    }

    async fn create_post(
        &self,
        ctx: &Context<'_>,
        input: CreatePostDTO,
    ) -> Result<PostDTO, GQLError> {
        let res = self.post_service.create_post(input).await?.into();
        Ok(res)
    }

    async fn add_friend(&self, ctx: &Context<'_>, input: AddFriendDTO) -> Result<bool, GQLError> {
        self.friendship_service.add_friend(input).await?;
        Ok(true)
    }

    async fn create_like(&self, ctx: &Context<'_>, input: CreateLikeDTO) -> Result<bool, GQLError> {
        self.like_service.create_like(input).await?;
        Ok(true)
    }
}

pub type MySchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(
    user_service: Arc<UserService>,
    post_service: Arc<PostService>,
    friendship_service: Arc<FriendshipService>,
    like_service: Arc<LikeService>,
) -> MySchema {
    Schema::build(
        Query {
            user_service: user_service.clone(),
            post_service: post_service.clone(),
            friendship_service: friendship_service.clone(),
            like_service: like_service.clone(),
        },
        Mutation {
            user_service: user_service.clone(),
            post_service: post_service.clone(),
            friendship_service: friendship_service.clone(),
            like_service: like_service.clone(),
        },
        EmptySubscription,
    )
    .data(user_service.clone())
    .data(post_service.clone())
    .data(friendship_service.clone())
    .data(like_service.clone())
    .finish()
}
