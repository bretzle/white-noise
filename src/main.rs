#![windows_subsystem = "windows"]

mod app;
mod audio;
mod config;
mod tray;

use anyhow::*;
use app::Noise;
use std::path::PathBuf;

fn main() {
	if let Err(e) = run() {
		eprintln!("{:?}", e);
	}
}

/// Runs the program
fn run() -> Result<()> {
	let mut app = Noise::new()?;
	app.start()
}

pub fn home() -> PathBuf {
	home::home_dir().unwrap()
}
