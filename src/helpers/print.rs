use super::{file::file_open, colors::print};
use crate::{helpers::{self}, ActionType};

pub fn print_detail(title: &str, value: String, atype: ActionType, color: &str) {
    print!("    ");
    
    match atype {
        ActionType::Details => {
            print(&title, true, &(color.to_owned() + "_bold"));
            for _ in 0..(10 - title.len()) {
                print!(" ");
            }

            helpers::colors::print(" : ", true, "white_bold");

            print!("{}", &value);
        },
        ActionType::Delimiter => {
            print("-----------------------------", true, "white_bold");
        },
        ActionType::HostInfo => {
            print(&format!(
                "{}@{}",
                title.to_owned(), 
                value.to_owned()
            ), 
                true, 
                &(color.to_owned() + "_bold")
            );
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