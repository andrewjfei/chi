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
    pub async fn add_clipboard_data(date_time: DateTime<Local>, data: String) {
        let data_type: ClipboardDataType = Self::process(&data);

        let clipboard_data: ClipboardData = ClipboardData::new(date_time, data_type, data);

        // println!("{:#?}", clipboard_data);

        ClipboardDataRepository::create(&clipboard_data).await;
    }

    pub async fn fetch_clipboard_data(limit: u32) -> Vec<ClipboardData> {
        let clipboard_data_list: Vec<ClipboardData> =
            ClipboardDataRepository::retrieve(limit).await;

        // println!("{:#?}", clipboard_data_list);

        return clipboard_data_list;
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
