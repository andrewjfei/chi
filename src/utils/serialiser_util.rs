use chrono::{DateTime, Local};
use serde::Serializer;

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
