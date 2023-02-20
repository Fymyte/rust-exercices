mod book;
mod data;

use axum::{self, routing::get, Router, Json};
use data::print_data;

#[tokio::main]
async fn main() {
    Router::new().route("/books", get(list_books));
}

async fn list_books() -> Json<Vec<book::Book>> {

}
