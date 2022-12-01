use super::{file::file_open, colors::print};
use crate::{helpers::{self}, ActionType};

pub fn print_detail(title: &str, value: String, atype: ActionType, color: &str) {
    print!("    ");
    
    match atype {
        ActionType::Details => {
            helpers::colors::print(&title, true, color);
            for _ in 0..(10 - title.len()) {
                print!(" ");
            }

            helpers::colors::print(" : ", true, "white");

            print!("{}", &value);
        },
        ActionType::Delimiter => {
            print("-----------------------------", true, "white");
        },
        ActionType::HostInfo => {
            print(&format!(
                "{}@{}",
                title, 
                value
            ), true, color);
        }
    };
}

pub fn print_ponyline(line: u16, pony: &str, color: &str) {
    file_open(&format!("ponies/{}.txt", pony).to_string())
        .lines()
        .skip(line as usize)
        .take(1)
        .for_each(|line| {
            print(line, true, color);
        });
}