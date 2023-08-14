use mongodb::{
    bson::{de::Error, doc, from_bson, from_slice, to_bson, Bson, Document, RawDocument},
    options::FindOptions,
    Collection, Cursor,
};

use crate::{
    configs::database_config::DATABASE_CONFIG,
    models::{clipboard_data::ClipboardData, database_client::DatabaseClient},
};

pub struct ClipboardDataRepository {
    // no properties
}

impl ClipboardDataRepository {
    pub async fn create(clipboard_data: &ClipboardData) {
        // get clipboard data collection
        let collection: Collection<Document> = Self::get_collection().await;

        // convert clipboard data object to a document
        let doc: Document = Self::to_document(clipboard_data).unwrap();

        // insert document into collection
        collection.insert_one(doc, None).await.unwrap();
    }

    pub async fn retrieve(limit: u32) -> Vec<ClipboardData> {
        // get clipboard data collection
        let collection: Collection<Document> = Self::get_collection().await;

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
            // convert raw document to clipboard data model
            let clipboard_data: ClipboardData = Self::to_clipboard_data(cursor.current())
                .expect("failed to deserialise ClipboardData document");

            // add clipboard data model to list
            clipboard_data_list.push(clipboard_data);
        }

        return clipboard_data_list;
    }

    async fn get_collection() -> Collection<Document> {
        let db_client: &DatabaseClient = DatabaseClient::instance().await;
        return db_client.get_collection(DATABASE_CONFIG.get_collection());
    }

    fn to_document(clipboard_data: &ClipboardData) -> Result<Document, Error> {
        // serialise clipboard data model to bson
        let serialised_object: Bson =
            to_bson(clipboard_data).expect("error serialising clipboard data model to bson");

        // verify serialised object is a document
        return match serialised_object {
            Bson::Document(document) => Ok(document),
            _ => panic!("serialized object is not a document"),
        };
    }

    fn to_clipboard_data(raw_doc: &RawDocument) -> Result<ClipboardData, Error> {
        // deserialise raw document to bson
        let bson: Bson = from_slice::<Bson>(raw_doc.as_bytes())
            .expect("failed to convert raw document bytes to bson");

        // deserialise bson to clipboard data model
        return from_bson::<ClipboardData>(bson);
    }
}
