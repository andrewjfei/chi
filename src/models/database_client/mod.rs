use mongodb::{bson::Document, options::ClientOptions, Client, Collection, Database};
use tokio::sync::OnceCell;

use crate::configs::database_config::DB_CONFIG;

// global database client
static DB_CLIENT: OnceCell<DatabaseClient> = OnceCell::const_new();

#[derive(Debug)]
pub struct DatabaseClient {
    pub client: Client,
    pub db: Database,
}

impl DatabaseClient {
    pub async fn instance() -> &'static DatabaseClient {
        // get existing database client if exists otherwise establish new client connection
        let db_client: &DatabaseClient = DB_CLIENT
            .get_or_init(|| async {
                return Self::connect().await;
            })
            .await;

        return db_client;
    }

    pub fn get_collection(&self, name: &str) -> Collection<Document> {
        return self.db.collection(name);
    }

    // connect to database
    async fn connect() -> DatabaseClient {
        let mut client_options: ClientOptions =
            ClientOptions::parse(DB_CONFIG.get_uri().to_string())
                .await
                .expect("uri is invalid");

        // manually set an option
        client_options.app_name = Some(DB_CONFIG.get_db().to_string());

        // create database client connection
        let client: Client =
            Client::with_options(client_options).expect("client options are invalid");

        // select database to use
        let db: Database = client.database(DB_CONFIG.get_db());

        return DatabaseClient { client, db };
    }
}
