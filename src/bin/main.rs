use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenvy::dotenv;
use friendship::service::FriendshipService;
use like::service::LikeService;
use post::service::PostService;
use std::{error, sync::Arc};

/*
use restapi::{

    friendship::{add_friend, get_friends},
    like::{create_like, get_likes},
    post::{create_post, get_posts},
    user::{create_user, get_user},
};
*/
use sm::friendship;
use sm::graphql;
use sm::like;
use sm::post;
use sm::user;

use user::service::UserService;

use graphql::schema::{create_schema, MySchema};

use axum::response::{Html, IntoResponse};
use axum::{routing::get, Extension, Router};

async fn graphql_handler(schema: Extension<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let nats_client: async_nats::Client = async_nats::connect("demo.nats.io").await?;
    dotenv().ok();

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var("DATABASE_URL").expect("env var"),
    );
    let pool = Pool::builder(config).build().expect("pool build");

    let user_service = Arc::new(UserService::new(pool.clone()));
    let post_service = Arc::new(PostService::new(pool.clone()));
    let friendship_service = Arc::new(FriendshipService::new(pool.clone()));
    let like_service = Arc::new(LikeService::new(pool.clone(), nats_client));

    let schema = create_schema(user_service, post_service, friendship_service, like_service);

    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())

    /*

    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/", post(create_user))
        .with_state(user_service)
        .route("/users/:id/posts", get(get_posts))
        .route("/users/:id/posts", post(create_post))
        .with_state(post_service)
        .route("/users/:id/friends", get(get_friends))
        .route("/users/:id/friends", post(add_friend))
        .with_state(friedship_service)
        .route("/users/:id/posts/:id/likes", get(get_likes))
        .route("/users/:id/posts/:id/likes", post(create_like))
        .with_state(like_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())

     */
}
