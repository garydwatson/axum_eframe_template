use axum::{
    extract,
    routing::get,
    routing::post,
    Router,
};
use serde::Deserialize;
use serde::Serialize;

use shared_types::CreateUser;
use tower_http::{
    services::{ServeDir, ServeFile},
};



#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = 
        Router::new()
            //.route("/", get(|| async { "Hello, World!" }))
            .route("/hello", get(|| async { "yoda" }))
            .route("/users", post(create_user))
            .nest_service("/", ServeDir::new("../frontend/dist/"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_user(extract::Json(payload): extract::Json<shared_types::CreateUser>) -> extract::Json<CreateUser> {
    println!("#{:#?}", payload);

    extract::Json(payload)
}