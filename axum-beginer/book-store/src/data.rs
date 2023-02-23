use std::{collections::HashMap, sync::Mutex, thread};

use crate::response;
use crate::{book::Book, response::BookStoreResponse};
use axum::response::Html;
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

pub async fn put_book(extract::Json(book): extract::Json<Book>) -> BookStoreResponse<String> {
    tokio::spawn(async move {
        let mut data = DATA.lock().unwrap();
        data.insert(book.id as u32, book.clone());
        BookStoreResponse::Ok((StatusCode::OK, format!("Put book {book}").into()))
    })
    .await
    .unwrap()
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

pub async fn get_book_form(extract::Path(id): extract::Path<u32>) -> Html<String> {
    tokio::spawn(async move {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!(
                concat!(
                    "<form method=\"post\" action=\"/books/{}/form\">\n",
                    "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                    "<p><input name=\"title\" value=\"{}\"></p>\n",
                    "<p><input name=\"author\" value=\"{}\"></p>\n",
                    "<input type=\"submit\" value=\"Save\">\n",
                    "</form>\n"
                ),
                &book.id, &book.id, &book.title, &book.author
            ),
            None => format!("<p>Book id {id} not found</p>"),
        }
    })
    .await
    .unwrap()
    .into()
}

pub async fn post_book_form(form: extract::Form<Book>) -> Html<String> {
    let new_book = form.0;
    tokio::spawn(async move {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&(new_book.id as u32)) {
            data.insert(new_book.id as u32, new_book.clone());
            format!("<p>{new_book}</p>")
        } else {
            format!("Book id not found: {}", new_book.id)
        }
    })
    .await
    .unwrap()
    .into()
}

/// Delete a book from database
/// id: the id of the book to delete
/// return Ok if book was found, Err otherwise
pub async fn delete_book(extract::Path(id): extract::Path<u32>) -> BookStoreResponse<Book> {
    tokio::spawn(async move {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&id) {
            let book = data.get(&id).unwrap().clone();
            data.remove(&id);
            BookStoreResponse::Ok((StatusCode::OK, book.into()))
        } else {
            BookStoreResponse::Err((StatusCode::NOT_FOUND, format!("No book found with id {id}")))
        }
    })
    .await
    .unwrap()
}
