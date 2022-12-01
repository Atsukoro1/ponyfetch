use super::{file::file_open, colors::print_cyan_bold};
use crate::{helpers::{self, colors::print_bold}, ActionType};

pub fn print_detail(title: &str, value: String, atype: ActionType) {
    print!("    ");
    
    match atype {
        ActionType::Details => {
            helpers::colors::print_cyan_bold(&title, true);
            for _ in 0..(10 - title.len()) {
                print!(" ");
            }

            helpers::colors::print_bold(" : ", true);

            print!("{}", &value);
        },
        ActionType::Delimiter => {
            print_bold("-----------------------------", true);
        },
        ActionType::HostInfo => {
            print_bold(&format!(
                "{}@{}",
                title, 
                value
            ), true);
        }
    };
}

pub fn print_ponyline(line: u16, pony: &str) {
    file_open(&format!("ponies/{}.txt", pony).to_string())
        .lines()
        .skip(line as usize)
        .take(1)
        .for_each(|line| {
            print_cyan_bold(line, true);
        });
}