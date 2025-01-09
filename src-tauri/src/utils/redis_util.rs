use cached::proc_macro::{cached, once};
use cached::SizedCache;
use crypto::{digest::Digest, md5::Md5};
use r2d2::Pool;
use redis::{Client, Commands, Msg, PubSub};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use std::thread::sleep;
use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

use crate::utils::cache_util;

use super::response::R;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Key {
    pub nname: Option<String>,
    pub name: Option<String>,
    pub leaf: Option<bool>,
    pub redis_uri: Option<String>,
    pub db: Option<String>,
}

pub fn get_key_from_cache(
    base_key: String,
    key: String,
    redis_uri: String,
    db: String,
) -> Option<Vec<Key>> {
    let values: Option<HashSet<String>> =
        cache_util::get_cache(&(base_key.clone() + ":" + &db + ":" + &key.clone()));
    log::info!(
        "查询key：{}",
        &(base_key.clone() + ":" + &db + ":" + &key.clone())
    );
    match values {
        Some(valses) => {
            let mut result: Vec<Key> = vec![];
            for i in valses.iter() {
                if i.ends_with(":") {
                    result.push(Key {
                        nname: Some(i.to_string()),
                        name: Some(i.replace(&key, "")),
                        leaf: Some(false),
                        redis_uri: Some(redis_uri.clone()),
                        db: Some(db.clone()),
                    });
                } else {
                    result.push(Key {
                        nname: Some(i.to_string()),
                        name: Some(i.to_string()),
                        leaf: Some(true),
                        redis_uri: Some(redis_uri.clone()),
                        db: Some(db.clone()),
                    });
                }
            }
            log::info!("返回值：{:?}", result.clone());
            return Some(result);
        }
        None => {
            log::info!("缓存中没有获取到，准备查询reids");
        }
    };
    None
}

pub fn get_keys(redis_uri: String, key: String, db: String) -> R<Vec<Key>> {
    let mut hasher = Md5::new();
    hasher.input_str(&redis_uri);
    let md5 = hasher.result_str();
    let mut base_key = md5.clone();
    let mut r_key = String::from("");
    match key.strip_suffix("*") {
        Some(rest) => {
            r_key = rest.to_string();
        }
        None => {
            r_key = key.clone();
        }
    }
    if !r_key.is_empty() && !r_key.ends_with(":") {
        return R::success();
    }
    if let Some(val) = get_key_from_cache(
        base_key.clone(),
        r_key.clone(),
        redis_uri.clone(),
        db.clone(),
    ) {
        return R::data(Some(val));
    };

    if let Some(pool) = get_client(redis_uri.to_string()) {
        let mut connection = pool.get().unwrap();
        if !db.is_empty() {
            // 切换到数据库索引为 1
            redis::cmd("SELECT")
                .arg(db.clone())
                .query::<()>(&mut connection)
                .unwrap();
        }
        let mut cursor: u64 = 0;
        let mut keys: Vec<String> = vec![];
        let mut result: HashSet<String> = HashSet::new();
        //log::info!("查询地址，{}  查询key {}   数据库 {}", redis_uri, key, db);
        loop {
            // 执行 SCAN 命令
            let (next_cursor, batch): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(key.clone()) // 这里可以指定模式，例如 "user:*"
                .arg("COUNT")
                .arg(100) // 每次迭代获取 100 个键，可以根据需要调整
                .query(&mut connection)
                .unwrap();

            for i in batch.iter() {
                keys.push(i.to_string());
                let str = i.to_string();
                let mut target = str.clone();
                if let Some(rest) = str.strip_prefix(key.as_str()) {
                    target = rest.to_string();
                }
                if let Some(rest) = target.strip_prefix(":") {
                    let splits: Vec<&str> = rest.split(":").collect();
                    if splits.len() > 1 {
                        result.insert(splits[0].to_owned() + ":");
                    } else {
                        result.insert(splits[0].to_string());
                    }
                } else {
                    let splits: Vec<&str> = target.split(":").collect();
                    if splits.len() > 1 {
                        result.insert(splits[0].to_owned() + ":");
                    } else {
                        result.insert(splits[0].to_string());
                    }
                }
            }
            if next_cursor == 0 {
                break;
            }
            cursor = next_cursor;
        }
        if keys.is_empty() {
            return R::success();
        }

        base_key = base_key.clone() + ":" + &db;
        if !r_key.is_empty() {
            base_key = base_key.clone() + ":" + &r_key.clone();
        }
        let mut keys_t = keys.clone();
        thread::spawn(move || {
            keys_t.sort();
            group_key_to_cache(keys_t, base_key.clone());
        });

        let mut result: Vec<String> = result.iter().cloned().collect();
        // 按字母顺序升序排序
        result.sort();
        let mut val: Vec<Key> = vec![];
        for i in result.iter() {
            if i.ends_with(":") {
                val.push(Key {
                    nname: Some(i.to_string()),
                    name: Some(i.replace(&key, "")),
                    leaf: Some(false),
                    redis_uri: Some(redis_uri.clone()),
                    db: Some(db.clone()),
                });
            } else {
                val.push(Key {
                    nname: Some(i.to_string()),
                    name: Some(i.replace(&key, "")),
                    leaf: Some(true),
                    redis_uri: Some(redis_uri.clone()),
                    db: Some(db.clone()),
                });
            }
        }
        return R::data(Some(val));
    } else {
        log::error!("链接redis失败！");
        R::fail("链接redis失败！".to_string())
    }
}

pub fn get_value(redis_uri: String, key: String, db: String) -> R<String> {
    let mut r_key = String::from("");
    match key.strip_suffix("*") {
        Some(rest) => {
            r_key = rest.to_string();
        }
        None => {
            r_key = key.clone();
        }
    }
    if r_key.is_empty() {
        log::info!("key为空！");
        return R::success();
    }

    if let Some(pool) = get_client(redis_uri.to_string()) {
        let mut connection = pool.get().unwrap();
        log::info!("查询库：{}", db);
        if !db.is_empty() {
            // 切换到数据库索引为 1
            redis::cmd("SELECT")
                .arg(db.clone())
                .query::<()>(&mut connection)
                .unwrap();
        }
        log::info!("查询key： {}", r_key);
        let rest: Result<String, redis::RedisError> = connection.get(r_key.clone());
        match rest {
            Ok(rest) => {
                log::info!("返回值： {}", rest);
                return R::data(Some(rest));
            }
            Err(e) => {
                log::error!("{:?}", e);
                return R::fail(format!("获取key【{}】的返回值失败！", r_key));
            }
        }
    } else {
        return R::fail("链接redis失败！".to_string());
    }
}

fn group_key_to_cache(keys: Vec<String>, base_key: String) {
    for i in keys.iter() {
        let target = i.to_string();
        // 将分割结果收集到 Vec 中
        let splits: Vec<&str> = target.split(":").collect();
        let mut key = "".to_string();
        for (index, text) in splits.iter().enumerate() {
            let value: Option<HashSet<String>> =
                cache_util::get_cache(&(base_key.clone() + ":" + &key));
            //log::info!("{} 查询到的数据：{:?}", base_key.clone() + ":" + &key.clone(), value.clone());
            let mut values: HashSet<String>;
            match value {
                Some(value) => {
                    values = value;
                }
                None => {
                    values = HashSet::new();
                }
            }
            let temp_key = key.clone();
            if index + 1 == splits.len() {
                key = key + text.to_owned();
                values.insert(key.clone());
            } else {
                key = key + text.to_owned() + ":";
                values.insert(key.clone());
            }
            //log::info!("{} 合并后数据：{:?}", base_key.clone() + ":" + &temp_key.clone(), values.clone());
            cache_util::set_cache(base_key.clone() + ":" + &temp_key, values);
        }
    }
}

//获取使用的db
fn get_use_db_num(redis_uri: String) -> HashMap<usize, usize> {
    if let Some(pool) = get_client(redis_uri.clone()) {
        let mut connection = pool.get().unwrap();
        // 手动执行 INFO 命令
        let info: String = redis::cmd("INFO")
            .arg("keyspace")
            .query(&mut connection)
            .unwrap();
        // 打印原始返回结果
        println!("INFO keyspace response: \n{}", info);
        // 找到所有已使用的数据库
        let db_info: HashMap<usize, usize> = info
            .lines()
            .filter(|line| line.starts_with("db"))
            .filter_map(|line| {
                // 示例: "db0:keys=5,expires=1,avg_ttl=0"
                let parts: Vec<&str> = line.split(':').collect();
                if let Some(db_name) = parts.get(0) {
                    let db_num = db_name.trim_start_matches("db").parse::<usize>().ok()?;
                    if let Some(details) = parts.get(1) {
                        // 从 "keys=5,expires=1,avg_ttl=0" 提取 keys 的值
                        let keys_info = details.split(',').find(|s| s.starts_with("keys="))?;
                        let keys_num = keys_info.split('=').nth(1)?.parse::<usize>().ok()?;
                        return Some((db_num, keys_num));
                    }
                }
                None
            })
            .collect();
        db_info
    } else {
        HashMap::new()
    }
}

//获取全部的db
pub fn get_all_db_num(redis_uri: String) -> R<HashMap<usize, usize>> {
    let mut result: HashMap<usize, usize> = HashMap::new();
    if let Some(pool) = get_client(redis_uri.clone()) {
        let mut connection = pool.get().unwrap();
        // 查询配置的数据库数量
        let config: Vec<String> = redis::cmd("CONFIG")
            .arg("GET")
            .arg("databases")
            .query(&mut connection)
            .unwrap();
        let total_databases: usize = config
            .get(1) // 第一个值是 "databases"，第二个值是数量
            .and_then(|s| s.parse().ok())
            .unwrap_or(160); // 默认 16

        println!("Total configured databases: {}", total_databases);
        for i in 0..total_databases {
            result.insert(i, 0);
        }
        let use_map = get_use_db_num(redis_uri);
        for (k, v) in use_map {
            result.insert(k, v);
        }
        R::data(Some(result))
    } else {
        for i in 0..16 {
            result.insert(i, 0);
        }
        R::fail_data("链接redis失败！".to_string(), Some(result))
    }
}

pub fn get_client(redis_uri: String) -> Option<Pool<Client>> {
    log::info!("redis地址：{}", redis_uri);
    if let Some(pool) = get_client_cache(redis_uri.clone()) {
        Some(pool)
    } else {
        let client = redis::Client::open(redis_uri);
        match client {
            Ok(client) => {
                let pool = r2d2::Pool::builder()
                    .connection_timeout(Duration::from_secs(1))
                    .build(client);
                match pool {
                    Ok(pool) => {
                        return Some(pool);
                    }
                    Err(e) => {
                        log::error!("打开redis链接失败！{}", e);
                        return None;
                    }
                }
            }
            Err(e) => {
                log::error!("打开redis链接失败！{}", e);
                return None;
            }
        }
    }
}

//#[once(option = true, time = 600)]
#[cached(
    ty = "SizedCache<String, Option<Pool<Client>>>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}", redis_uri) }"#
)]
pub fn get_client_cache(redis_uri: String) -> Option<Pool<Client>> {
    log::info!("redis地址：{}", redis_uri);
    let client = redis::Client::open(redis_uri);
    match client {
        Ok(client) => {
            let pool = r2d2::Pool::builder()
                .connection_timeout(Duration::from_secs(1))
                .build(client);
            match pool {
                Ok(pool) => {
                    return Some(pool);
                }
                Err(e) => {
                    log::error!("打开redis链接失败！{}", e);
                    return None;
                }
            }
        }
        Err(e) => {
            log::error!("打开redis链接失败！{}", e);
            return None;
        }
    }
}

#[test]
pub fn test() {
    let redis_uri = "redis://192.168.4.49:6379/0";
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let client = redis::Client::open(redis_uri).unwrap();
        let mut con = client.get_connection().unwrap();

        // 订阅所有键的变化
        let mut pubsub: PubSub = con.as_pubsub();
        pubsub.subscribe("__keyspace@0__:*").unwrap();

        // 持续监听并处理事件
        loop {
            let msg = pubsub.get_message().unwrap();
            let payload: String = msg.get_payload().unwrap();
            println!("Received keyspace notification: {}", payload);

            // 根据收到的消息内容处理
            if payload.contains("set") {
                println!("Key was set or updated.");
            } else if payload.contains("del") {
                println!("Key was deleted.");
            } else if payload.contains("expired") {
                println!("Key expired.");
            }
        }
    });
}
