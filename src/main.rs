// register modules
mod configs;
mod enums;
mod models;
mod repositories;
mod services;
mod utils;

use chrono::{Local, DateTime};
use clipboard::{ClipboardContext, ClipboardProvider};
use dotenv::dotenv;
use std::error::Error;

use repositories::clipboard_data_repository::ClipboardDataRepository;
use services::clipboard_data_service::ClipboardDataService;

#[tokio::main]
async fn main() {
    // load .env variables as environment variables
    dotenv().expect("failed to load .env file");

    // create a clipboard context
    let mut clipboard_context: ClipboardContext = ClipboardProvider::new()
        .expect("failed to create clipboard context");

    // empty previous data string
    let mut previous_data: String = String::new();

    loop {
        // retrieve clipboard data
        let result: Result<String, Box<dyn Error>> = clipboard_context.get_contents();

        match result {
            Ok(data) => {
                // validate data to prevent persisting duplicates
                if data != previous_data {
                    // get current local date time
                    let date_time: DateTime<Local> = Local::now();

                    // persist data to database
                    ClipboardDataService::add_clipboard_data(date_time, data.clone()).await;

                    // set current data as previous data
                    previous_data = data;
                }
            },
            Err(err) => {
                // ignore empty clipboard data error
                if !err.to_string().contains("returned empty") {
                    println!("{}", err)
                }
            },
        }
    }

    // ClipboardDataService::fetch_clipboard_data(5).await;
}
