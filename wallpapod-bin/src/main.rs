use anyhow::ensure;
use args::Commands;
use clap::Parser;
use colorz::Colorize;
use config::Config;
use wallpaper::Mode;
use crate::apod::ApodResponse;

mod apod;
mod args;
mod config;

type Result<T> = anyhow::Result<T>;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const URI: &str = "https://api.nasa.gov/planetary/apod";

fn main() -> Result<()> {
    let args = args::Cli::parse();
    let mut cfg = confy::load::<Config>(PKG_NAME, Some(config::CFG_FILE))?;

    if let Some(api) = args.api {
        cfg.api_key = api;
        confy::store(PKG_NAME, Some(config::CFG_FILE), &cfg)?;
        println!("Stored APOD API key in '{}'!", "config".yellow())
    }

    ensure!(!cfg.api_key.is_empty(), "Missing API key");

    match args.command {
        Some(Commands::Pull) => pull(cfg.api_key),
        None => Ok(())
    }
}

fn pull(api_key: String) -> Result<()> {
    let uri = format!("{}?api_key={}", URI, api_key);
    let body = reqwest::blocking::get(uri)?.text()?;
    let resp = serde_json::from_str::<ApodResponse>(&body)?;
    wallpaper::set_mode(Mode::Crop).unwrap();
    wallpaper::set_from_url(&resp.hdurl).unwrap();
    Ok(())
}