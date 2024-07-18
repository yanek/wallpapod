use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets your APOD API key
    #[arg(short, long, value_name = "API_KEY")]
    pub api: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Pulls and set a new wallpaper
    Pull,
}
