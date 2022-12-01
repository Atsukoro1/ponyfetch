// make const with color names and match them with unicode symbol for color
const COLORS : [(&str, &str); 17] = [
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
];

pub fn print(text: &str, inline: bool, color: &str) {
    let mut color = color;
    if color == "rainbowdash" {
        color = "bright_magenta";
    }

    let color = color;
    let color = COLORS.iter().find(|(name, _)| name == &color).unwrap().1;
    
    if inline {
        print!("{}{}", color, text);
    } else {
        println!("{}{}", color, text);
    }
}