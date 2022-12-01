use std::{fs::File, io::Read};
use std::process::Command;

#[cfg(target_os = "linux")]
pub fn get_hostname() -> String {
    let mut hostname = String::new();

    let mut f = File::open("/etc/hostname").unwrap();
    f.read_to_string(&mut hostname).unwrap();

    hostname.pop();

    hostname
}

#[cfg(target_os = "linux")]
pub fn get_user() -> String {
    Command::new("whoami")
        .output()
        .expect("Failed to execute whoami")
        .stdout
        .iter()
        .map(|&c| c as char)
        .collect::<String>()
        .trim()
        .to_string()
}

#[cfg(target_os = "linux")] 
pub fn get_distro() -> String {
    use std::rc::Rc;

    let mut distro: Rc<String> = Rc::new(String::new());
    let mut temp_buf: String = String::new();

    let mut file = File::open("/etc/os-release").unwrap();
    file.read_to_string(&mut temp_buf).unwrap();

    let lines: &Vec<&str> = &temp_buf.lines().collect();
    
    lines.into_iter().for_each(|line| {
        if line.contains("PRETTY_NAME") {
            distro = Rc::new(
                line.split("=")
                    .collect::<Vec<&str>>()[1].to_string()
                    .replace("\"", "")
            );
        }

        if line.contains("BUILD_ID") {
            distro = Rc::new(
                format!("{} ({})", distro, 
                    line.split("=")
                        .collect::<Vec<&str>>()[1].to_string()
                        .replace("\"", "")
                )
            );
        }
    });

    distro.to_string()
}

#[cfg(target_os = "linux")]
pub fn get_uptime() -> String {
    let mut temp_buf: String = String::new();

    let mut file = File::open("/proc/uptime").unwrap();
    file.read_to_string(&mut temp_buf).unwrap();

    let uptime: u128 = temp_buf.split(".").collect::<Vec<&str>>()[0].parse().unwrap();

    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;

    format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
}

#[cfg(target_os = "linux")]
pub fn get_shell() -> String {
    let mut final_str = String::new();
    let mut temp_buf: String = String::new();

    let mut f = File::open("/etc/passwd").unwrap();
    f.read_to_string(&mut temp_buf).unwrap();

    let lines: &Vec<&str> = &temp_buf.lines().collect();

    lines.into_iter().for_each(|line| {
        if line.contains(&get_user()) {
            final_str = line.split(":")
                .collect::<Vec<&str>>()[6]
                .to_string();
        }
    });

    final_str
}