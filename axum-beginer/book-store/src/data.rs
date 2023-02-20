use std::{collections::HashMap, sync::Mutex};

use crate::book::{self, Book};
use once_cell::sync::Lazy;

static DATA: Lazy<Mutex<HashMap<usize, Book>>> = Lazy::new(|| Mutex::new({
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
}));

pub async fn print_data() {
    println!("data: {:?}", DATA.lock().unwrap());
}
