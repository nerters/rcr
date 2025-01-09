// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use env_logger::{Builder, Target};

fn main() {
    Builder::new()
        .target(Target::Stdout) // 输出到标准输出
        .parse_filters("info") // 设置最低的日志级别
        .init();

    rcr_lib::run()
}
