// register modules
mod models;
mod daos;
mod repositories;
mod utils;
mod configs;

use std::env::{args, Args}; // import method to read program input arguments

use daos::clipboard_data_dao::ClipboardDataDao;
use repositories::clipboard_data_repository::ClipboardDataRepository;
use dotenv::dotenv;
use chrono::Local;

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

    let timestamp = Local::now();
    let data_type = "TEXT".to_string();
    let data = "The quick brown fox jumped over the lazy brown dog.".to_string();
  
    let clipboard_data_dao = ClipboardDataDao::new(timestamp, data_type, data);

    println!("{:#?}", clipboard_data_dao);

    ClipboardDataRepository::save(&clipboard_data_dao).await;
}
