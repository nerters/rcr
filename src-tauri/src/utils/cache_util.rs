use std::sync::Arc;
use std::time::{Duration, Instant};
use cached::{Cached, SizedCache};
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::sync::Mutex;
use tokio::time::interval;

// 定义缓存结构，包含数据和过期时间
struct CacheValue {
    value: Box<dyn std::any::Any + Send + Sync>,
    expire_at: Option<Instant>, // `None` 代表永不过期
}

// 定义全局缓存，使用 Mutex 包裹以支持并发访问
static GLOBAL_CACHE: Lazy<Arc<Mutex<SizedCache<String, CacheValue>>>> =
    Lazy::new(|| Arc::new(Mutex::new(SizedCache::with_size(100))));

#[tokio::main]
pub async fn set_cache<T: 'static + Send + Sync + Clone + std::fmt::Debug>(key: String, value: T) {
    set_cache_(key, value, None).await
}

#[tokio::main]
pub async fn set_cache_timeout<T: 'static + Send + Sync + Clone + std::fmt::Debug>(key: String, value: T, ttl: u64) {
    set_cache_(key, value, Some(Duration::from_secs(ttl))).await
}


/// 设置缓存，并支持设置过期时间
/// - `ttl`: `Some(Duration)` 代表缓存超时时间，`None` 代表永不过期
pub async fn set_cache_<T: 'static + Send + Sync + Clone + std::fmt::Debug>(key: String, value: T, ttl: Option<Duration>) {
    let expire_at = ttl.map(|t| Instant::now() + t); // 计算过期时间
    let mut cache = GLOBAL_CACHE.lock().await;
    cache.cache_set(key, CacheValue {
        value: Box::new(value),
        expire_at,
    });
}

#[tokio::main]
pub async fn get_cache<T: 'static + Clone>(key: &str) -> Option<T> {
    get_cache_(key).await
}



/// 获取缓存，若过期则返回 `None`
pub async fn get_cache_<T: 'static + Clone>(key: &str) -> Option<T> {
    let mut cache = GLOBAL_CACHE.lock().await;
    
    if let Some(entry) = cache.cache_get(key) {
        if let Some(expire_at) = entry.expire_at {
            if Instant::now() > expire_at {
                cache.cache_remove(key); // 过期后删除
                return None;
            }
        }
        return entry.value.downcast_ref::<T>().cloned();
    }
    None
}

// 使用正则表达式进行模糊匹配
#[tokio::main]
// 使用正则表达式进行模糊匹配
pub async fn get_cache_with_regex_match<T: 'static + Clone>(pattern: &str) -> Option<T> {
    get_cache_with_regex_match_(pattern).await
}


/// 使用正则匹配查询缓存
pub async fn get_cache_with_regex_match_<T: 'static + Clone>(pattern: &str) -> Option<T> {
    let re = Regex::new(pattern).unwrap();
    let mut cache = GLOBAL_CACHE.lock().await;

    // 先收集符合的 key，防止遍历过程中修改缓存
    let matching_keys: Vec<String> = cache
        .key_order()
        .filter(|key| re.is_match(key))
        .cloned()
        .collect();

    for key in matching_keys {
        // 这里不能直接调用 cache_get，因为 cache 之前是不可变借用
        if let Some(entry) = cache.cache_get(&key) {
            if let Some(expire_at) = entry.expire_at {
                if Instant::now() > expire_at {
                    cache.cache_remove(&key); // 删除过期缓存
                    continue;
                }
            }
            return entry.value.downcast_ref::<T>().cloned();
        }
    }
    None
}


pub async fn clean_expired_cache() {
    let mut cache = GLOBAL_CACHE.lock().await;
    let now = Instant::now();

    cache.retain(|_key, entry| {
        if let Some(expire_at) = entry.expire_at {
            expire_at > now // 只保留未过期的
        } else {
            true // 如果没有过期时间，保留
        }
    });
}

/// 定期清理过期缓存
pub async fn start_cleanup_task(interval_secs: u64) {
    let mut interval = interval(Duration::from_secs(interval_secs));
    
    loop {
        interval.tick().await;
        clean_expired_cache().await;
    }
}
