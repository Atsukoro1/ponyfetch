use super::{file::file_open, colors::print};
use crate::{helpers::{self}, ActionType};

pub fn print_detail(title: &str, value: String, atype: ActionType, color: &str) {
    print!("    ");
    
    match atype {
        ActionType::Details => {
            print(&title, true, &(color.to_owned() + "_bold"));
            for _ in 0..(12 - title.len()) {
                print!(" ");
            }

            helpers::colors::print(" : ", true, "white_bold");

            print!("{}", &value);
        },
        ActionType::Delimiter => {
            print("-----------------------------", true, "white");
        },
        ActionType::HostInfo => {
            print(
                title, 
                true, 
                &(color.to_owned() + "_bold")
            );
            print("@", true, "white_bold");
            print(
                &value, 
                true, 
                &(color.to_owned() + "_bold")
            );
        },

        ActionType::Colors => {
            print("████", true, "black");
            print("████", true, "red");
            print("████", true, "green");
            print("████", true, "yellow");
            print("████", true, "blue");
            print("████", true, "magenta");
        }
    };
}

pub fn print_ponyline(line: u16, pony: &str, color: &str) {
    let mut lines = pony.split("\n");
    let line = lines.nth(line as usize).unwrap().to_string();

    print(&line, true, color);
}