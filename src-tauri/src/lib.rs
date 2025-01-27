use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    WebviewUrl, WebviewWindowBuilder,
};
use utils::redis_util::{ttt, zzz};

pub mod serve;
pub mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    utils::creat_folder::creat_base_folder();

    //ttt();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .menu(|handle| {
            Menu::with_items(
                handle,
                &[
                    #[cfg(target_os = "macos")]
                    &Submenu::with_items(
                        handle,
                        "rcr",
                        true,
                        &[
                            &MenuItem::new(handle, "关于", true, None::<&str>)?,
                           
                            &MenuItem::new(handle, "你好", true, None::<&str>)?,
                            &PredefinedMenuItem::close_window(handle, Some("退出"))?,
                        ],
                    )?,
                    #[cfg(target_os = "macos")]
                    &Submenu::with_items(
                        handle,
                        "文件",
                        true,
                        &[
                            #[cfg(target_os = "macos")]
                            &MenuItem::new(handle, "你好", true, None::<&str>)?,
                        ],
                    )?,
                ],
            )
        })
        .setup(|app| {
            let handle = app.handle();
            // 定义菜单
            let save_menu_item = MenuItem::new(handle, "保存", true, None::<&str>)?;
            let save_menu = Submenu::new(handle, "保存", true)?;
            let menu = Menu::with_items(
                handle,
                &[&Submenu::with_items(
                    handle,
                    "文件",
                    true,
                    &[&save_menu_item],
                )?],
            )?;

            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

            let win_builder = win_builder.title("rcr");
            let win_builder = win_builder.inner_size(800.0, 600.0);
            #[cfg(target_os = "macos")]
            let win_builder = win_builder.menu(menu);

            // 仅在 macOS 时设置透明标题栏
            #[cfg(target_os = "macos")]
            use tauri::TitleBarStyle;
            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

            let window = win_builder.build();
            match window {
                Ok(window) => {
                    // 仅在构建 macOS 时设置背景颜色
                    #[cfg(target_os = "macos")]
                    {
                        use cocoa::appkit::{NSColor, NSWindow};
                        use cocoa::base::{id, nil};

                        let ns_window = window.ns_window().unwrap() as id;
                        unsafe {
                            let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                                nil,
                                50.0 / 255.0,
                                158.0 / 255.0,
                                163.5 / 255.0,
                                1.0,
                            );
                            ns_window.setBackgroundColor_(bg_color);
                        }
                    }
                }
                Err(e) => {
                    log::error!("构建错误！ {}", e);
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            serve::redis_serve::get_keys,
            serve::redis_serve::get_value,
            serve::redis_serve::get_db_num,
            serve::redis_serve::pubsub,
            serve::redis_serve::reset_client,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
