mod db;

use std::env::{args, Args}; // import method to read program input arguments

use db::client::connect;
use dotenv::dotenv;

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

    // attmept to connect to mongodb client
    let client = connect().await;

    // list the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await.unwrap() {
        println!("{}", db_name);
    }
}
