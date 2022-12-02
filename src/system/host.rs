#[cfg(target_os = "linux")]
use crate::helpers::file::file_open;
#[cfg(target_os = "linux")]
use std::{fs::File, io::Read};

#[cfg(any(target_os = "windows", target_os = "linux"))]
use std::process::Command;

#[cfg(target_os = "windows")]
pub fn get_hostname() -> String {
    let mut hostname = String::new();

    let output = Command::new("reg")
        .args(&["query", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ComputerName", "/v", "ComputerName"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if line.contains("ComputerName") {
            hostname = line.split_whitespace().last().unwrap().to_string();
        }
    }

    hostname
}

#[cfg(target_os = "windows")]
pub fn get_user() -> String {
    let mut user = String::new();

    let output = Command::new("reg")
        .args(&["query", "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList", "/s"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if line.contains("ProfileImagePath") {
            user = line.split_whitespace().last().unwrap().to_string();
            user = user.split("\\").last().unwrap().to_string();
        }
    }

    user
}

#[cfg(target_os = "windows")]
pub fn get_distro() -> String {
    let mut distro = String::new();

    let output = Command::new("reg")
        .args(&["query", "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", "/v", "ProductName"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if line.contains("ProductName") {
            distro = line.split_whitespace().last().unwrap().to_string();
        }
    }

    distro
}

#[cfg(target_os = "windows")]
pub fn get_uptime() -> String {
    let mut uptime = String::new();

    let output = Command::new("reg")
        .args(&["query", "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", "/v", "NUMBER_OF_PROCESSORS"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if line.contains("NUMBER_OF_PROCESSORS") {
            uptime = line.split_whitespace().last().unwrap().to_string();
        }
    }

    uptime
}

#[cfg(target_os = "windows")]
pub fn get_shell() -> String {
    let mut shell = String::new();

    let output = Command::new("reg")
        .args(&["query", "HKCU\\Console", "/v", "FaceName"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if line.contains("FaceName") {
            shell = line.split_whitespace().last().unwrap().to_string();
        }
    }

    if shell == "Lucida Console" {
        shell = "PowerShell".to_string();
    } else {
        shell = "CMD".to_string();
    }

    shell
}

#[cfg(target_os = "windows")]
pub fn get_resolution() -> String {
    let mut temp_horiz = String::new();
    let mut temp_vert = String::new();

    let output = Command::new("wmic")
        .args(&[
            "path", 
            "Win32_VideoController", 
            "get", 
            "CurrentVerticalResolution,CurrentHorizontalResolution", 
            "/format:value"
        ])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    let split_by_equals = |item: &str| -> String {
        item.split("=")
            .collect::<Vec<&str>>()
            .iter()
            .nth(1)
            .unwrap()
            .trim()
            .to_string()
    };

    for line in output.lines() {
        if line.contains("CurrentHorizontalResolution") {
            temp_horiz = split_by_equals(line);
        }

        if line.contains("CurrentVerticalResolution") {
            temp_vert = split_by_equals(line);
        }
    }

    format!(
        "{}x{}", 
        temp_horiz, 
        temp_vert
    ).to_string()
}

#[cfg(target_os = "linux")]
pub fn get_hostname() -> String {
    let mut hostname = file_open("/etc/hostname");
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
    let temp_buf: String = file_open("/proc/uptime");

    let uptime: u128 = temp_buf.split(".")
        .collect::<Vec<&str>>()[0]
        .parse()
        .unwrap();

    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;

    format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
}

#[cfg(target_os = "linux")]
pub fn get_shell() -> String {
    let temp_buf: String = file_open("/etc/passwd");
    let mut final_str = String::new();

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

#[cfg(target_os = "linux")]
pub fn get_resolution() -> String {
    let mut final_str = String::new();

    let output = Command::new("xrandr")
        .output()
        .expect("Failed to execute xrandr");

    let output = String::from_utf8(output.stdout).unwrap();

    let lines: &Vec<&str> = &output.lines().collect();

    lines.into_iter().for_each(|line| {
        if line.contains(" connected") {
            final_str = line.split(" ")
                .collect::<Vec<&str>>()[2]
                .to_string();
        }
    });

    final_str
}

#[cfg(target_os = "linux")]
pub fn get_init_system() -> String {
    Command::new("ps")
        .args(&["-p", "1", "-o", "comm="])
        .output()
        .unwrap()
        .stdout
        .iter()
        .map(|&c| c as char)
        .collect::<String>()
        .trim().to_string()
}