use std::sync::Arc;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenvy::dotenv;
use friendship::service::FriendshipService;
use post::service::PostService;
use restapi::{
    friendship::{add_friend, get_friends},
    post::{create_post, get_posts},
    user::{create_user, get_user},
};

use user::service::UserService;

mod friendship;
pub mod post;
mod restapi;
pub mod schema;
mod user;
pub mod utils;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenv().ok();

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var("DATABASE_URL").expect("env var"),
    );
    let pool = Pool::builder(config).build().expect("pool build");

    let user_service = Arc::new(UserService::new(pool.clone()));
    let post_service = Arc::new(PostService::new(pool.clone()));
    let friedship_service = Arc::new(FriendshipService::new(pool.clone()));

    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/", post(create_user))
        .with_state(user_service)
        .route("/users/:id/posts", get(get_posts))
        .route("/users/:id/posts", post(create_post))
        .with_state(post_service)
        .route("/users/:id/friends", get(get_friends))
        .route("/users/:id/friends", post(add_friend))
        .with_state(friedship_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
