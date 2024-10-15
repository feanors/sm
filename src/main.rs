use std::sync::Arc;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenvy::dotenv;
use restapi::user::{create_user, get_user};

use user::service::UserService;

mod restapi;
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

    let user_service = Arc::new(UserService::new(pool));

    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/", post(create_user))
        .with_state(user_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
