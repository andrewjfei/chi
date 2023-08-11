use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use serde::Serialize;

use crate::{
    enums::clipboard_data_type::ClipboardDataType, utils::serialiser_util::SerialiserUtil,
};

#[derive(Debug, Serialize)]
pub struct ClipboardDataDao {
    _id: ObjectId,
    #[serde(serialize_with = "SerialiserUtil::date_time_serialiser")]
    date_time: DateTime<Local>,
    #[serde(serialize_with = "SerialiserUtil::clipboard_data_type_serialiser")]
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
