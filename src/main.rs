pub mod routes;

use crate::routes::{html::html, rps::rps};
use axum::{self, routing::get, Router};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(html).post(rps));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .expect("failed to run the server");
}
