use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub fn get_pony_path() -> String {
    if std::env::current_exe().unwrap().to_str().unwrap().contains("target") {
        PathBuf::from("ponies/")
            .to_str()
            .unwrap()
            .to_string()
    } else {
        PathBuf::from("/usr/share/ponyfetch/ponies/")
            .to_str()
            .unwrap()
            .to_string()
    }
}

#[cfg(target_os = "windows")]
pub fn get_pony_path() -> String {
    if std::env::current_exe().unwrap().to_str().unwrap().contains("target") {
        PathBuf::from("ponies/")
            .to_str()
            .unwrap()
            .to_string()
    } else {
        PathBuf::from("C:\\Program Files\\Ponyfetch\\ponies\\")
            .to_str()
            .unwrap()
            .to_string()
    }
}