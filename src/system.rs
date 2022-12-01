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
        .collect()
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
pub fn get_ipaddr() -> String {
    // Get current using network interface
    let mut f = File::open("/proc/net/route").unwrap();
    let mut intr = String::new();
    f.read_to_string(&mut intr).unwrap();

    let lines: &Vec<&str> = &intr.lines().collect();
    let mut interface = String::new();

    lines.into_iter().for_each(|line| {
        if line.contains("00000000") {
            interface = line.split("\t").collect::<Vec<&str>>()[0].to_string();
        }
    });

    interface
}