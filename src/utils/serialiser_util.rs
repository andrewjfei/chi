use chrono::{DateTime, Local};
use serde::{Serialize, Serializer};

use crate::enums::clipboard_data_type::ClipboardDataType;

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
        return date_time.to_rfc3339().serialize(serialiser);
    }

    pub fn serialise_clipboard_data_type<S>(
        data_type: &ClipboardDataType,
        serialiser: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let clipboard_data_type_str = match data_type {
            ClipboardDataType::Email => "EMAIL",
            ClipboardDataType::File => "FILE",
            ClipboardDataType::Image => "IMAGE",
            ClipboardDataType::Link => "LINK",
            ClipboardDataType::Text => "TEXT",
        };

        // return serialiser.serialize_str(&data_type_str);

        return clipboard_data_type_str.serialize(serialiser);
    }
}
