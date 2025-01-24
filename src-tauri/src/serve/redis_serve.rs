use std::collections::HashMap;

use crate::utils::{self, redis_util::{zzz, Key}, response::R};

//let redis_uri = "redis://192.168.5.126:6379/1";
//let redis_uri_v1 = "redis://:123123@192.168.4.49:6379/0";
//let redis_uri_v2 = "redis://:9R9SVEnjbS1r9SFHa8@192.168.2.151:6111/10";
#[tauri::command]
pub fn get_keys(redis_uri: String, key: String, db: String) -> R<Vec<Key>> {
    utils::redis_util::get_keys(redis_uri, key, db)
}

#[tauri::command]
pub fn get_value(redis_uri: String, key: String, db: String) -> R<String> {
    utils::redis_util::get_value(redis_uri, key, db)
}

#[tauri::command]
pub fn get_db_num(redis_uri: String) -> R<HashMap<usize, usize>> {
    log::info!("查询数据库中的db库数量：");
    utils::redis_util::get_all_db_num(redis_uri)
}


#[tauri::command]
pub async fn pubsub() {
    zzz().await;
}