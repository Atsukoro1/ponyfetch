#[derive(clap::Parser)]
#[clap(
    version = "1.0", 
    author = "Atsukoro1",
    about = "Just a simple cross-platform neofetch for all the bronies out there."
)]
pub struct Args {
    #[clap(short, long, required = false, default_value = "rainbowdash")]
    /// Defines what ASCII pony to print.
    pub pony: String,

    #[clap(short, long, required = false, default_value = "0")]
    /// Defines what color to print the output it
    pub color: String
}