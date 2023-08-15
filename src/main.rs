// register modules
mod configs;
mod constants;
mod enums;
mod models;
mod repositories;
mod services;
mod utils;

use chrono::{DateTime, Local};
use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use dotenv::dotenv;
use std::{
    error::Error,
    io::{stdout, Write},
};

use crate::{
    constants::{CHI_END_CMD_CHAR, CHI_NEW_CMD_PROMPT, CHI_SHOW_HISTORY_CMD},
    models::clipboard_data::ClipboardData,
    services::clipboard_data_service::ClipboardDataService,
};

#[tokio::main]
async fn main() {
    // load .env variables as environment variables
    dotenv().expect("failed to load .env file");

    // spawn async thread to handle terminal cli commands
    tokio::spawn(async {
        loop {
            // prompt user that application is ready to accept commands
            print!("{}", CHI_NEW_CMD_PROMPT);

            // force print all buffered text into the standout output stream
            stdout()
                .flush()
                .expect("failed to flush standard output text");

            // intialise command string
            let mut cmd: String = String::new();

            // loop through all input characters after user hits enter
            loop {
                if let Event::Key(KeyEvent {
                    code,
                    modifiers: _,
                    state: _,
                    kind: _,
                }) = read().unwrap()
                {
                    match code {
                        KeyCode::Char(char) => {
                            if char == CHI_END_CMD_CHAR {
                                break; // exit the inner loop
                            } else {
                                cmd.push(char);
                            }
                        }
                        _ => {} // throw away anything that isn't a character
                    }
                }
            }

            // process command
            if cmd.trim().to_lowercase().eq(CHI_SHOW_HISTORY_CMD) {
                // fetch clipboard data from database
                let clipboard_data_list: Vec<ClipboardData> =
                    ClipboardDataService::fetch_clipboard_data(5).await;

                for clipboard_data in clipboard_data_list {
                    println!(
                        "{} {}",
                        clipboard_data.get_date_time(),
                        clipboard_data.get_data()
                    );
                }
            }
        }
    });

    // todo: fix issue where clipboard thread does not always persist, could be issue with mongodb connection

    // create a clipboard context
    let mut clipboard_context: ClipboardContext =
        ClipboardProvider::new().expect("failed to create clipboard context");

    // empty previous data string
    let mut prev_data: String = String::new();

    loop {
        // retrieve clipboard data
        let res: Result<String, Box<dyn Error>> = clipboard_context.get_contents();

        match res {
            Ok(data) => {
                // validate data to prevent persisting duplicates
                if data != prev_data {
                    // get current local date time
                    let date_time: DateTime<Local> = Local::now();

                    // persist data to database
                    ClipboardDataService::add_clipboard_data(date_time, data.clone()).await;

                    println!("{}", data);

                    // set current data as previous data
                    prev_data = data;
                }
            }
            Err(err) => {
                // ignore empty clipboard data error
                if !err.to_string().contains("returned empty") {
                    println!("{}", err)
                }
            }
        }
    }
}
