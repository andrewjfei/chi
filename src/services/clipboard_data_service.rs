use chrono::{DateTime, Local};

use crate::{
    daos::clipboard_data_dao::ClipboardDataDao,
    repositories::clipboard_data_repository::ClipboardDataRepository,
};

pub struct ClipboardDataService {
    // no properties
}

impl ClipboardDataService {
    pub async fn create_clipboard_data(date_time: DateTime<Local>, data: String) {
        let clipboard_data_dao = ClipboardDataDao::new(date_time, "TEXT".to_string(), data);

        println!("{:#?}", clipboard_data_dao);

        ClipboardDataRepository::save(clipboard_data_dao).await;
    }
}
