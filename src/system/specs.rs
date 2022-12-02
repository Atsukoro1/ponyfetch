use crate::helpers::file::file_open;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

#[cfg(target_os = "windows")]
pub fn get_cpu() -> String {
    use std::process::Command;

    let mut cpu = String::new();

    let output = Command::new("wmic")
        .args(&["cpu", "get", "name"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if !line.contains("Name") && line.trim().len() > 0 {
            cpu = line.to_string();
        }
    }

    cpu
}

#[cfg(target_os = "windows")]
pub fn get_ram_used() -> String {
    use std::process::Command;

    let mut ram_used = String::new();

    let output = Command::new("wmic")
        .args(&["OS", "get", "FreePhysicalMemory"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if !line.contains("FreePhysicalMemory") && line.trim().len() > 0 {
            ram_used = line.to_string();
        }
    }

    ram_used
}

#[cfg(target_os = "windows")]
pub fn get_kernel() -> String {
    use std::process::Command;

    let mut kernel = String::new();

    let output = Command::new("wmic")
        .args(&["OS", "get", "Caption"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if !line.contains("Caption") && line.trim().len() > 0 {
            kernel = line.to_string();
        }
    }

    kernel
}

#[cfg(target_os = "linux")]
pub fn get_cpu() -> String {
    let mut cpu: Rc<String> = Rc::new(String::new());
    let mut temp_buf: String = String::new();

    let mut file = File::open("/proc/cpuinfo").unwrap();
    file.read_to_string(&mut temp_buf).unwrap();

    let lines: &Vec<&str> = &temp_buf.lines().collect();

    lines.into_iter().for_each(|line| {
        if line.contains("model name") {
            cpu = Rc::new(
                line.split(":")
                    .collect::<Vec<&str>>()[1].to_string()
                    .replace("\t", "")
            );
            cpu = Rc::new(cpu.replacen(" ", "", 1));
        }
    });

    cpu.to_string()
}

#[cfg(target_os = "linux")]
pub fn get_ram_used() -> String {
    let temp_buf: String = file_open("/proc/meminfo");

    let lines: &Vec<&str> = &temp_buf.lines().collect();

    let mut total: u128 = 0;
    let mut available: u128 = 0;

    lines.into_iter().for_each(|line| {
        if line.contains("MemTotal") {
            total = eval_ram(line.to_string());
        } else if line.contains("MemAvailable") {
            available = eval_ram(line.to_string());
        }
    });

    format!(
        "{}M / {}M",
        total - available,
        total
    )
}

fn eval_ram(line: String) -> u128 {
    let kbs: u128 = line.split(":")
        .collect::<Vec<&str>>()[1].to_string()
        .replace("\t", "")
        .replace("kB", "")
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();

    kbs / 1000
}

#[cfg(target_os = "linux")]
pub fn get_kernel() -> String {
    let temp_buf: String = file_open("/proc/version");

    temp_buf.split(" ")
        .collect::<Vec<&str>>()[2]
        .to_string()
}

#[cfg(target_os = "windows")]
pub fn get_gpu() -> String {
    use std::process::Command;

    let mut gpu = String::new();

    let output = Command::new("wmic")
        .args(&["path", "win32_VideoController", "get", "Name"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if !line.contains("Name") && line.trim().len() > 0 {
            gpu = line.to_string();
        }
    }

    gpu
}