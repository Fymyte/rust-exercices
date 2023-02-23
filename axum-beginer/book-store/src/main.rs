mod book;
mod data;
mod response;

use std::net::SocketAddr;

use axum::{self, routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/books", get(data::list_books))
        .route("/books/:id", get(data::get_book));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
