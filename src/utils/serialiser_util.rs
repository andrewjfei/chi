use chrono::{DateTime, Local};
use serde::Serializer;

use crate::enums::clipboard_data_type::ClipboardDataType;

pub fn date_time_serialiser<S>(
    date_time: &DateTime<Local>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date_time_str = date_time.to_rfc3339();
    return serializer.serialize_str(&date_time_str);
}

pub fn clipboard_data_type_serialiser<S>(
    data_type: &ClipboardDataType,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let data_type_str = match data_type {
        ClipboardDataType::File => "FILE",
        ClipboardDataType::Image => "IMAGE",
        ClipboardDataType::Link => "LINK",
        ClipboardDataType::Text => "TEXT",
    };

    return serializer.serialize_str(&data_type_str);
}
