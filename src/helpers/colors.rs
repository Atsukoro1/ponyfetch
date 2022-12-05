use crate::helpers::console::enable_ansi_support;

pub const COLORS : [(&str, &str); 34] = [
    ("black", "\u{001b}[30m"),
    ("red", "\u{001b}[31m"),
    ("green", "\u{001b}[32m"),
    ("yellow", "\u{001b}[33m"),
    ("blue", "\u{001b}[34m"),
    ("magenta", "\u{001b}[35m"),
    ("cyan", "\u{001b}[36m"),
    ("white", "\u{001b}[37m"),
    ("bright_black", "\u{001b}[90m"),
    ("bright_red", "\u{001b}[91m"),
    ("bright_green", "\u{001b}[92m"),
    ("bright_yellow", "\u{001b}[93m"),
    ("bright_blue", "\u{001b}[94m"),
    ("bright_magenta", "\u{001b}[95m"),
    ("bright_cyan", "\u{001b}[96m"),
    ("bright_white", "\u{001b}[97m"),
    ("cyan_bold", "\u{001b}[36m\u{001b}[1m"),
    ("bright_cyan_bold", "\u{001b}[96m\u{001b}[1m"),
    ("reset", "\u{001b}[0m"),
    ("bold", "\u{001b}[1m"),
    ("black_bold", "\u{001b}[30m\u{001b}[1m"),
    ("red_bold", "\u{001b}[31m\u{001b}[1m"),
    ("green_bold", "\u{001b}[32m\u{001b}[1m"),
    ("yellow_bold", "\u{001b}[33m\u{001b}[1m"),
    ("blue_bold", "\u{001b}[34m\u{001b}[1m"),
    ("magenta_bold", "\u{001b}[35m\u{001b}[1m"),
    ("white_bold", "\u{001b}[37m\u{001b}[1m"),
    ("bright_black_bold", "\u{001b}[90m\u{001b}[1m"),
    ("bright_red_bold", "\u{001b}[91m\u{001b}[1m"),
    ("bright_green_bold", "\u{001b}[92m\u{001b}[1m"),
    ("bright_yellow_bold", "\u{001b}[93m\u{001b}[1m"),
    ("bright_blue_bold", "\u{001b}[94m\u{001b}[1m"),
    ("bright_magenta_bold", "\u{001b}[95m\u{001b}[1m"),
    ("bright_white_bold", "\u{001b}[97m\u{001b}[1m"),
];

pub fn print(text: &str, inline: bool, color: &str) {
    let color = COLORS.iter().find(
        |(name, _)| name == &color
    ).unwrap().1;

    #[cfg(windows)]
    enable_ansi_support();

    if inline {
        print!("{}{}", color, text);
    } else {
        println!("{}{}", color, text);
    }

    print!("{}", COLORS[18].1);
}