use std::time::{SystemTime, UNIX_EPOCH};
use log::{error, info};
use storage::{MysqlStorage, Storage};

pub async fn clear_decided_task(mysql_connect_url: String) -> Result<u32, String> {
    let storage = MysqlStorage::new(mysql_connect_url.clone());
    let timestamp_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let timestamp_yesterday = timestamp_now - 86400;

    let mut error_count = 0;

    match storage.delete_operator_decided_before_timestamp(timestamp_yesterday) {
        Ok(deleted_number) => {
            info!("successful delete_operator_decided_before_timestamp deleted_number: {}", deleted_number);
        }
        Err(error_info) => {
            error!("failed delete_operator_decided_before_timestamp error_info: {}", error_info);
            error_count += 1;
        }
    }

    match storage.delete_decided_before_timestamp(timestamp_yesterday) {
        Ok(deleted_number) => {
            info!("successful delete_decided_before_timestamp deleted_number: {}", deleted_number);
        }
        Err(error_info) => {
            info!("failed delete_decided_before_timestamp error_info: {}", error_info);
            error_count += 1;
        }
    }

    return Ok(error_count)

}