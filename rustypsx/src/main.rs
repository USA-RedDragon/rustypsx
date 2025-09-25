#![warn(clippy::all)]
#![forbid(unsafe_code)]

mod config;
mod cpu;
mod display;
mod memory;
mod psx;

use clap::Parser;

fn main() -> Result<(), pixels::Error> {
    let config = config::RawConfig::parse().clean();
    let ps1 = psx::PS1::new();

    display::run_with_gui(ps1, &config)
}
