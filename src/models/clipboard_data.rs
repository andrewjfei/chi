use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{
    enums::clipboard_data_type::ClipboardDataType,
    utils::{deserialiser_util::DeserialiserUtil, serialiser_util::SerialiserUtil},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardData {
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

impl ClipboardData {
    pub fn new(
        date_time: DateTime<Local>,
        data_type: ClipboardDataType,
        data: String,
    ) -> ClipboardData {
        return ClipboardData {
            _id: ObjectId::new(),
            date_time,
            data_type,
            data,
        };
    }
}
