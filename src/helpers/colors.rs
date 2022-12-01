pub fn print_cyan(text: &str, inline: bool) {
    if inline {
        print!("\x1b[36m{}\x1b[0m", text);
    } else {
        println!("\x1b[36m{}\x1b[0m", text);
    }
}

pub fn print_cyan_bold(text: &str, inline: bool) {
    if inline {
        print!("\x1b[36m\x1b[1m{}\x1b[0m", text);
    } else {
        println!("\x1b[36m\x1b[1m{}\x1b[0m", text);
    }
}

pub fn print_bold(text: &str, inline: bool) {
    if inline {
        print!("\x1b[1m{}\x1b[0m", text);
    } else {
        println!("\x1b[1m{}\x1b[0m", text);
    }
}