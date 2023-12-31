use std::env;

use lazy_static::lazy_static;

use crate::constants::{
    MONGODB_COLLECTION, MONGODB_DB, MONGODB_DEFAULT_COLLECTION, MONGODB_DEFAULT_DB,
    MONGODB_DEFAULT_URI, MONGODB_URI,
};

lazy_static! {
  // lazy load database config
  pub static ref DB_CONFIG: DatabaseConfig = DatabaseConfig::load();
}

#[derive(Debug)]
pub struct DatabaseConfig {
    uri: String,
    db: String,
    collection: String,
}

impl DatabaseConfig {
    // private constructor
    fn load() -> DatabaseConfig {
        // retrieve mongodb uri from environment variable
        let uri = env::var(MONGODB_URI).unwrap_or(MONGODB_DEFAULT_URI.to_string());

        // retrieve mongodb db from environment variable
        let db = env::var(MONGODB_DB).unwrap_or(MONGODB_DEFAULT_DB.to_string());

        // retrieve mongodb collection from environment variable
        let collection =
            env::var(MONGODB_COLLECTION).unwrap_or(MONGODB_DEFAULT_COLLECTION.to_string());

        return DatabaseConfig {
            uri,
            db,
            collection,
        };
    }

    pub fn get_uri(&self) -> &str {
        return &self.uri;
    }

    pub fn get_db(&self) -> &str {
        return &self.db;
    }

    pub fn get_collection(&self) -> &str {
        return &self.collection;
    }
}
