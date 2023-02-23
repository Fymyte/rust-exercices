use std::{collections::HashMap, sync::Mutex, thread};

use crate::response;
use crate::{book::Book, response::BookStoreResponse};
use axum::{extract, http::StatusCode, Json};
use once_cell::sync::Lazy;

static DATA: Lazy<Mutex<HashMap<u32, Book>>> = Lazy::new(|| {
    Mutex::new({
        HashMap::from([
            (
                1,
                Book {
                    id: 1,
                    title: "Antigone".to_owned(),
                    author: "Sophocles".to_owned(),
                },
            ),
            (
                2,
                Book {
                    id: 2,
                    title: "Beloved".to_owned(),
                    author: "Toni Morisson".to_owned(),
                },
            ),
            (
                3,
                Book {
                    id: 3,
                    title: "Candide".to_owned(),
                    author: "Voltaire".to_owned(),
                },
            ),
        ])
    })
});

#[allow(dead_code)]
pub async fn print_data() {
    println!("data: {:?}", DATA.lock().unwrap());
}

pub async fn list_books() -> Json<Vec<Book>> {
    let data = DATA.lock().unwrap();
    data.values()
        .map(|book| book.clone())
        .collect::<Vec<_>>()
        .into()
}

pub async fn get_book(extract::Path(id): extract::Path<u32>) -> BookStoreResponse<Book> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => BookStoreResponse::Ok((StatusCode::OK, book.clone().into())),
            None => BookStoreResponse::Err((
                StatusCode::NOT_FOUND,
                format!("No book found with id {id}"),
            )),
        }
    })
    .join()
    .unwrap()
}
