use chrono::{DateTime, Local};
use serde::{Serialize, Serializer};

use crate::{
    constants::{
        ENUM_CLIPBOARD_DATA_EMAIL, ENUM_CLIPBOARD_DATA_FILE, ENUM_CLIPBOARD_DATA_IMAGE,
        ENUM_CLIPBOARD_DATA_LINK, ENUM_CLIPBOARD_DATA_TEXT,
    },
    enums::clipboard_data_type::ClipboardDataType,
};

#[cfg(test)]
mod tests;

pub struct SerialiserUtil {
    // no properties
}

impl SerialiserUtil {
    pub fn serialise_date_time<S>(
        date_time: &DateTime<Local>,
        serialiser: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let date_time_str = date_time.to_rfc3339();
        return date_time_str.serialize(serialiser);
    }

    pub fn serialise_clipboard_data_type<S>(
        data_type: &ClipboardDataType,
        serialiser: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let clipboard_data_type_str = match data_type {
            ClipboardDataType::Email => ENUM_CLIPBOARD_DATA_EMAIL,
            ClipboardDataType::File => ENUM_CLIPBOARD_DATA_FILE,
            ClipboardDataType::Image => ENUM_CLIPBOARD_DATA_IMAGE,
            ClipboardDataType::Link => ENUM_CLIPBOARD_DATA_LINK,
            ClipboardDataType::Text => ENUM_CLIPBOARD_DATA_TEXT,
        };

        return clipboard_data_type_str.serialize(serialiser);
    }
}
