use chrono::{DateTime, Local};

use crate::{
    enums::clipboard_data_type::ClipboardDataType, models::clipboard_data::ClipboardData,
    repositories::clipboard_data_repository::ClipboardDataRepository,
    utils::clipboard_data_type_util::ClipboardDataTypeUtil,
};

pub struct ClipboardDataService {
    // no properties
}

impl ClipboardDataService {
    pub async fn create_clipboard_data(date_time: DateTime<Local>, data: String) {
        let data_type = Self::process(&data);

        let clipboard_data = ClipboardData::new(date_time, data_type, data);

        println!("{:#?}", clipboard_data);

        ClipboardDataRepository::create(&clipboard_data).await;
    }

    fn process(data: &str) -> ClipboardDataType {
        if ClipboardDataTypeUtil::is_email(data) {
            return ClipboardDataType::Email;
        }

        if ClipboardDataTypeUtil::is_link(data) {
            return ClipboardDataType::Link;
        }

        // default to text
        return ClipboardDataType::Text;
    }
}
