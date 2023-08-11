// register modules
mod configs;
mod daos;
mod enums;
mod models;
mod repositories;
mod services;
mod utils;

use std::env::{args, Args}; // import method to read program input arguments

use chrono::Local;
use dotenv::dotenv;
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
    let data = "johnsmith".to_string();

    ClipboardDataService::create_clipboard_data(date_time, data).await;
}
