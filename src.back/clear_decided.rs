use std::time::{SystemTime, UNIX_EPOCH};
use log::info;
use storage::{MysqlStorage, Storage};

pub async fn clear_decided_task(mysql_connect_url: String) -> Result<u32, String> {
    let storage = MysqlStorage::new(mysql_connect_url.clone());
    let timestamp_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let timestamp_yesterday = timestamp_now - 86400;

    return match storage.delete_operator_decided_before_timestamp(timestamp_yesterday) {
        Ok(deleted_number) => {
            info!("successful delete_operator_decided_before_timestamp deleted_number: {}", deleted_number);
            Ok(0)
        }
        Err(error_info) => {
            Err(format!("failed delete_operator_decided_before_timestamp error_info: {}", error_info))
        }
    }

}