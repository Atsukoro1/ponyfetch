use crate::helpers::colors::COLORS;

#[derive(Debug)]
pub struct Arguments {
    pub help: bool,
    pub color: String,
    pub list: bool,
    pub pony: String   
}

impl Arguments {
    fn validate_color(color: String) -> String {
        for c in COLORS.iter() {
            if c.0 == color {
                return color.to_string();
            }
        };

        Self::print_err("Invalid color provided.");
        std::process::exit(1);
    }

    fn validate_pony(pony: String) -> String {
        let ponies = crate::helpers::paths::get_ponies();

        for p in ponies.iter() {
            if p == &pony {
                return pony.to_string();
            }
        };

        Self::print_err("Invalid pony provided.");
        std::process::exit(1);
    }   

    fn print_help() {
        println!("Usage: ponyfetch [OPTION]...");
        println!("Prints a pony with system information.");
        println!("
            -h, --help      Display this help and exit
            -c=<color>, --color=<color>     Set the color of the pony
            -l=<color>, --list=<color>      List all available ponies
            -p=<color>, --pony=<color>      Set the pony to display
        ");
        
        std::process::exit(0);
    }

    fn print_ponies() {
        let ponies = crate::helpers::paths::get_ponies();

        println!("Available ponies:");
        for pony in ponies.iter() {
            println!("    {}", pony);
        }

        std::process::exit(0);
    }

    fn print_err(err: &str) {
        println!("Error: {}", err);
        println!("Usage: ponyfetch [OPTION]...");
        println!("Try 'ponyfetch --help' for more information.");
        std::process::exit(1);
    }

    fn get_args(arg: String) -> String {
        let args = arg.split("=").collect::<Vec<&str>>();

        if args.len() < 2 {
            Self::print_err("Invalid argument provided.");
        };

        args[1].to_string()
    }

    pub fn parse() -> Arguments {
        let mut args = Arguments {
            help: false,
            list: false,
            color: String::from(""),
            pony: String::from("")
        };

        let args_vec: Vec<String> = std::env::args().collect();

        args_vec.into_iter().for_each(|arg| {
            match arg {
                arg if arg == "--help" || arg == "-h" => args.help = true,

                arg if arg.contains("--color") || arg.contains("-c") => {
                    args.color = Self::validate_color(
                        Self::get_args(arg)
                    );
                },

                arg if arg.contains("--pony") || arg.contains("-p") => {
                    args.pony = Self::validate_pony(
                        Self::get_args(arg)
                    );
                },

                arg if arg == "--list" || arg == "-l" => args.list = true,
                _ => ()
            }
        });

        if args.color == "" {
            args.color = String::from("blue");
        }

        if args.pony == "" {
            args.pony = String::from("derpy_large");
        }

        if args.help {
            Self::print_help();
        }

        if args.list {
            Self::print_ponies();
        }

        args
    }
}