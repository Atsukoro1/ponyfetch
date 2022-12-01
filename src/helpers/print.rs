use crate::{helpers, system};

pub fn print_detail(title: &str, value: String) {
    helpers::colors::print_cyan_bold(&title, true);
    for _ in 0..(8 - title.len()) {
        print!(" ");
    }

    helpers::colors::print_bold(" : ", true);

    print!("{}", &value);
    println!();
}