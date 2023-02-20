use std::{net::SocketAddr, collections::HashMap};

use axum::{extract::{Path, Query}, http, response::Html, routing::get, Router, Server, Json};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(not_found)
        .route("/", get(root_handler))
        .route("/selftest", get(self_test_handler))
        .route("/echo-uri", get(echo_uri_handler))
        .route("/items", get(items_list_handler))
        .route("/items/:id", get(items_handler))
        .route("/json", get(json_get).put(json_put))
        .route(
            "/allin",
            get(get_allin)
                .put(put_allin)
                .patch(patch_allin)
                .post(post_allin)
                .delete(delete_allin),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct MyJsonObject {
    one: String,
    second: String,
}

async fn json_get() -> Json<Value> {
    json!({"one": "first", "second": "two"}).into()
}

async fn json_put(Json(data): Json<MyJsonObject>) -> String {
    format!("PUT demo JSON data: {:?}", data)
}

async fn items_list_handler(Query(items): Query<HashMap<String, String>>) -> String {
    format!("found items {:?}", items)
}

async fn items_handler(Path(id): Path<usize>) -> String {
    format!("found uuid {id}")
}

async fn put_allin(uri: http::Uri) -> String {
    format!("PUT {uri}")
}

async fn get_allin(uri: http::Uri) -> String {
    format!("GET {uri}")
}

async fn patch_allin(uri: http::Uri) -> String {
    format!("PATCH {uri}")
}

async fn post_allin(uri: http::Uri) -> String {
    format!("POST {uri}")
}

async fn delete_allin(uri: http::Uri) -> String {
    format!("DELETE {uri}")
}

async fn echo_uri_handler(uri: http::Uri) -> String {
    format!("Requested URI: {uri}")
}

async fn self_test_handler() -> (http::StatusCode, Html<String>) {
    (
        http::StatusCode::OK,
        Html(format!(
            "If you see this message, your config is probably fine"
        )),
    )
}

async fn root_handler() -> &'static str {
    "Hello, World!"
}

async fn not_found(uri: http::Uri) -> (http::StatusCode, Html<String>) {
    (
        http::StatusCode::NOT_FOUND,
        Html(format!("<h1>404</h1><p>page {uri} not found")),
    )
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("Shutting down")
}
