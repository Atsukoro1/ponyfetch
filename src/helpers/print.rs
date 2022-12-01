use crate::{helpers, system};

use super::file::file_open;

pub fn print_detail(title: &str, value: String) {
    helpers::colors::print_cyan_bold(&title, true);
    for _ in 0..(8 - title.len()) {
        print!(" ");
    }

    helpers::colors::print_bold(" : ", true);

    print!("{}", &value);
    println!();
}

pub fn print_ponyline(line: u16, pony: &str) {
    file_open(&format!("ponies/{}.txt", pony).to_string())
        .lines()
        .skip(line as usize)
        .take(1)
        .for_each(|line| {
            print!("{}", line);
        });
}