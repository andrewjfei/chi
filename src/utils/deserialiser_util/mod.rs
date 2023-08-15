use chrono::{DateTime, Local};
use serde::{de::Error, Deserialize, Deserializer};

use crate::{
    constants::{
        ENUM_CLIPBOARD_DATA_EMAIL, ENUM_CLIPBOARD_DATA_FILE, ENUM_CLIPBOARD_DATA_IMAGE,
        ENUM_CLIPBOARD_DATA_LINK, ENUM_CLIPBOARD_DATA_TEXT,
    },
    enums::clipboard_data_type::ClipboardDataType,
};

pub struct DeserialiserUtil {
    // no properties
}

impl DeserialiserUtil {
    pub fn deserialise_date_time<'de, D>(deserialiser: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_time_str: String = Deserialize::deserialize(deserialiser)
            .expect("failed to desrialise date time data type");

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
            .expect("failed to desrialise clipboard data type enum");

        return match clipboard_data_type_str.as_str() {
            str if str == ENUM_CLIPBOARD_DATA_EMAIL => Ok(ClipboardDataType::Email),
            str if str == ENUM_CLIPBOARD_DATA_FILE => Ok(ClipboardDataType::File),
            str if str == ENUM_CLIPBOARD_DATA_IMAGE => Ok(ClipboardDataType::Image),
            str if str == ENUM_CLIPBOARD_DATA_LINK => Ok(ClipboardDataType::Link),
            str if str == ENUM_CLIPBOARD_DATA_TEXT => Ok(ClipboardDataType::Text),
            _ => Err(Error::custom(format_args!(
                "invalid clipboard data type {}",
                clipboard_data_type_str
            ))),
        };
    }
}
