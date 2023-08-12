use mongodb::{
    bson::{doc, from_bson, from_slice, to_bson, Bson, Document, RawDocument, de::Error},
    options::FindOptions,
    Collection, Cursor,
};

use crate::{
    configs::database_config::DATABASE_CONFIG,
    models::{database_client::DatabaseClient, clipboard_data::ClipboardData},
};

pub struct ClipboardDataRepository {
    // no properties
}

impl ClipboardDataRepository {
    pub async fn save(clipboard_data_dao: ClipboardData) {
        // get clipboard data collection
        let collection = Self::get_collection().await;

        // serialise clipboard data dao to bson
        let serialised_object =
            to_bson(&clipboard_data_dao).expect("error serialising clipboard data object to bson");

        // verify serialised object is a document
        let doc = match serialised_object {
            Bson::Document(doc) => doc,
            _ => panic!("serialized object is not a document"),
        };

        // insert document into collection
        collection.insert_one(doc, None).await.unwrap();
    }

    pub async fn retrieve(limit: u32) -> Vec<ClipboardData> {
        // get clipboard data collection
        let collection = Self::get_collection().await;

        // configuration for sorting the documents before retrieval
        let sort_options: Document = doc! {
            "date_time": -1, // sort date time by descending order (i.e. newest to oldest)
        };

        // additional options when retrieving documents
        let options: FindOptions = FindOptions::builder()
            .sort(sort_options)
            .limit(limit as i64) // limit the number of documents retrieved
            .build();

        // retrieve all documents which match the filter and options provided
        let mut cursor: Cursor<Document> = collection
            .find(None, options)
            .await
            .expect("failed to retrieve documents");

        let mut clipboard_data_list: Vec<ClipboardData> = Vec::new();

        // iterate through the cursor
        while cursor.advance().await.unwrap() {
            let clipboard_data: ClipboardData = Self::to_clipboard_data(cursor.current())
                .expect("failed to deserialise ClipboardData document");
            clipboard_data_list.push(clipboard_data);
        }

        return clipboard_data_list;
    }

    async fn get_collection() -> Collection<Document> {
        let db_client = DatabaseClient::instance().await;
        return db_client.get_collection(DATABASE_CONFIG.get_collection());
    }

    fn to_clipboard_data(raw_doc: &RawDocument) -> Result<ClipboardData, Error> {
        let bson: Bson = from_slice::<Bson>(raw_doc.as_bytes())
            .expect("failed to convert RawDocument bytes to Bson");
        return from_bson::<ClipboardData>(bson);
    }
}
