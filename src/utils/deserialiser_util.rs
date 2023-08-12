use chrono::{DateTime, Local};
use serde::{de::Error, Deserialize, Deserializer};

use crate::enums::clipboard_data_type::ClipboardDataType;

pub struct DeserialiserUtil {
    // no properties
}

impl DeserialiserUtil {
    pub fn deserialise_date_time<'de, D>(deserialiser: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_time_str: String = Deserialize::deserialize(deserialiser)
            .expect("failed to desrialise DateTime data type");
        return DateTime::parse_from_rfc3339(&date_time_str)
            .map_err(Error::custom)
            .map(|date_time| date_time.with_timezone(&Local));
    }

    pub fn deserialise_clipboard_data_type<'de, D>(
        deserialiser: D,
    ) -> Result<ClipboardDataType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let clipboard_data_type_str: String = Deserialize::deserialize(deserialiser)
            .expect("failed to desrialise ClipboardDataType enum");

        return match clipboard_data_type_str.as_str() {
            "EMAIL" => Ok(ClipboardDataType::Email),
            "FILE" => Ok(ClipboardDataType::File),
            "IMAGE" => Ok(ClipboardDataType::Image),
            "LINK" => Ok(ClipboardDataType::Link),
            "TEXT" => Ok(ClipboardDataType::Text),
            _ => Err(Error::custom(format_args!(
                "invalid ClipboardDataType {}",
                clipboard_data_type_str
            ))),
        };
    }
}
