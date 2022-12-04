use std::path::PathBuf;

pub fn get_ponies() -> Vec<String> {
    let mut ponies: Vec<String> = Vec::new();
    let pony_files = std::fs::read_dir(get_pony_path()).unwrap();

    pony_files.for_each(|file| {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        let file_name = file_name.split(".").collect::<Vec<&str>>()[0].to_string();
        ponies.push(file_name);
    });

    ponies
}

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