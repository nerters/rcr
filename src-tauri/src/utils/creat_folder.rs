use directories::BaseDirs;

pub fn creat_base_folder() {
    println!("初始化数据库线程池--------开始-------");

    let mut path = String::from("");
    if let Some(base_dirs) = BaseDirs::new() {
        let appdata_dir = base_dirs.data_dir();
        path += appdata_dir.to_str().unwrap();
        println!("AppData directory: {:?}", appdata_dir);
    }
    path += "/com.rcr.app";
    //let path = "C:\\Users\\tjw1t\\AppData\\Roaming\\tk.tools";

    if let Ok(metadata) = std::fs::metadata(path.clone()) {
        if !metadata.is_file() {
            std::fs::create_dir_all(path.clone());
            std::fs::File::create(path.clone());
        }
    } else {
        std::fs::create_dir_all(path.clone());
        std::fs::File::create(path.clone());
    }
}
