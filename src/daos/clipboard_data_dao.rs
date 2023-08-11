use chrono::{Local, DateTime};
use mongodb::bson::oid::ObjectId;
use serde::Serialize;

use crate::utils::serialiser_util::date_time_serialiser;

#[derive(Debug, Serialize)]
pub struct ClipboardDataDao {
  _id: ObjectId,
  #[serde(serialize_with = "date_time_serialiser")]
  date_time: DateTime<Local>,
  data_type: String,
  data: String,
}

impl ClipboardDataDao {
  pub fn new(date_time: DateTime<Local>, data_type: String, data: String) -> ClipboardDataDao {
    return ClipboardDataDao {
      _id: ObjectId::new(),
      date_time,
      data_type,
      data,
    }
  }
}
