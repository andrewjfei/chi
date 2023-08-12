// register modules
mod configs;
mod enums;
mod models;
mod repositories;
mod services;
mod utils;

use chrono::Local;
use dotenv::dotenv;
use std::env::{args, Args};

use repositories::clipboard_data_repository::ClipboardDataRepository;
use services::clipboard_data_service::ClipboardDataService;

#[tokio::main]
async fn main() {
    // load .env variables as environment variables
    dotenv().expect("failed to load .env file");

    // load application arguments
    let args: Args = args();

    // print all arguments
    for arg in args {
        println!("{}", arg);
    }

    let date_time = Local::now();
    let data = "https://chi.com".to_string();

    ClipboardDataService::create_clipboard_data(date_time, data).await;
    let list = ClipboardDataRepository::retrieve(5).await;

    dbg!(list);
}
