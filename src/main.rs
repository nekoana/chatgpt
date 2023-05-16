#![feature(async_fn_in_trait)]
mod model;
mod server;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
