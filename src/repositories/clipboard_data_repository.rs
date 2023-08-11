use mongodb::{bson::{to_bson, Bson, Document}, Collection};

use crate::{
  configs::database_config::DATABASE_CONFIG, 
  daos::clipboard_data_dao::ClipboardDataDao,
  models::database_client::DatabaseClient,
};

pub struct ClipboardDataRepository {
  // no properties
}

impl ClipboardDataRepository {
  pub async fn save(clipboard_data_dao: &ClipboardDataDao) {  
    // get clipboard data collection
    let collection = Self::get_collection().await;
  
    // serialise clipboard data dao to bson
    let serialised_object = to_bson(&clipboard_data_dao)
      .expect("error serialising clipboard data object to bson");
  
    // verify serialised object is a document
    let doc = match serialised_object {
      Bson::Document(doc) => doc,
      _ => panic!("serialized object is not a document"),
    };
  
    // insert document into collection
    collection.insert_one(doc, None).await.unwrap();
  }

  async fn get_collection() -> Collection<Document> {
    let db_client = DatabaseClient::instance().await;
    return db_client.get_collection(DATABASE_CONFIG.get_collection());
  }
}
