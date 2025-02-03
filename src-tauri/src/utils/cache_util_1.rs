use std::sync::Arc;

use cached::{Cached, SizedCache};
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::sync::Mutex;

// 定义全局缓存，使用 Mutex 包裹以支持并发访问
static GLOBAL_CACHE: Lazy<Arc<Mutex<SizedCache<String, Box<dyn std::any::Any + Send + Sync>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(SizedCache::with_size(100))));

// 插入缓存
#[tokio::main]
pub async fn set_cache<T: 'static + Send + Sync + Clone + std::fmt::Debug>(key: String, value: T) {
    set_cache_(key, value).await
}
pub async fn set_cache_<T: 'static + Send + Sync + Clone + std::fmt::Debug>(key: String, value: T) {
    //log::info!("保存key：{}   -----  保存值：{:?}", key, value.clone());
    let mut cache = GLOBAL_CACHE.lock().await;
    cache.cache_set(key, Box::new(value));
}

// 获取缓存并尝试转换为具体类型
#[tokio::main]
pub async fn get_cache<T: 'static + Clone>(key: &str) -> Option<T> {
    //log::info!("查询key：{}", key);
    get_cache_(key).await
}

pub async fn get_cache_<T: 'static + Clone>(key: &str) -> Option<T> {
    //log::info!("查询key：{}", key);
    let mut cache = GLOBAL_CACHE.lock().await;
    if let Some(value) = cache.cache_get(key) {
        value.downcast_ref::<T>().cloned()
    } else {
        None
    }
}

// 使用正则表达式进行模糊匹配
#[tokio::main]
// 使用正则表达式进行模糊匹配
pub async fn get_cache_with_regex_match<T: 'static + Clone>(pattern: &str) -> Option<T> {
    get_cache_with_regex_match_(pattern).await
}

pub async fn get_cache_with_regex_match_<T: 'static + Clone>(pattern: &str) -> Option<T> {
    let re = Regex::new(pattern).unwrap();
    let mut cache = GLOBAL_CACHE.lock().await; // 获取 MutexGuard
                                               // 手动遍历缓存项
                                               // 通过 cache.cache_get_all() 获取所有键值对
    for key in cache.key_order() {
        if re.is_match(key) {
            // 正则匹配
            return get_cache_(&key).await;
        }
    }
    None
}
