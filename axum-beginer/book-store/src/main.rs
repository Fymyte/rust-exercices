mod book;
mod data;
mod response;

use std::net::SocketAddr;

use axum::{self, routing::get, Router, Server};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/books", get(data::list_books).put(data::put_book))
        .route("/books/:id", get(data::get_book).delete(data::delete_book))
        .route("/books/:id/form", get(data::get_book_form).post(data::post_book_form));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
