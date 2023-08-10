use std::env;

use mongodb::{Client, options::ClientOptions};

pub async fn connect() -> Client {
  // retrieve mongodb uri from environment variable
  let uri = env::var("MONGODB_URI")
    .unwrap_or("mongodb://localhost:27017".to_string());

  // parse a connection string into an options struct
  let mut client_options = ClientOptions::parse(uri)
    .await
    .expect("uri is invalid");

  // manually set an option
  client_options.app_name = Some("CHI".to_string());

  // get a handle to the deployment
  let client = Client::with_options(client_options).expect("client options are invalid");

  return client;
}