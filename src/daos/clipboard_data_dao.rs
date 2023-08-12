use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{
    enums::clipboard_data_type::ClipboardDataType,
    utils::{deserialiser_util::DeserialiserUtil, serialiser_util::SerialiserUtil},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardDataDao {
    _id: ObjectId,
    #[serde(
        serialize_with = "SerialiserUtil::serialise_date_time",
        deserialize_with = "DeserialiserUtil::deserialise_date_time"
    )]
    date_time: DateTime<Local>,
    #[serde(
        serialize_with = "SerialiserUtil::serialise_clipboard_data_type",
        deserialize_with = "DeserialiserUtil::deserialise_clipboard_data_type"
    )]
    data_type: ClipboardDataType,
    data: String,
}

impl ClipboardDataDao {
    pub fn new(
        date_time: DateTime<Local>,
        data_type: ClipboardDataType,
        data: String,
    ) -> ClipboardDataDao {
        return ClipboardDataDao {
            _id: ObjectId::new(),
            date_time,
            data_type,
            data,
        };
    }
}
