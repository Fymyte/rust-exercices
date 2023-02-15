#![allow(dead_code)]

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    foo().await
}

async fn foo() {
    println!("hello asyncroneously");
}
