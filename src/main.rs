// #![windows_subsystem = "windows"]

#[macro_use]
extern crate lazy_static;

mod app;
mod audio;
mod config;
mod tray;

use anyhow::*;
use app::Noise;
use config::Config;
use std::{
	fs::File,
	fs::{self},
	io::Write,
	path::PathBuf,
	sync::atomic::AtomicI32,
};

lazy_static! {
	static ref HOME: PathBuf = home::home_dir().unwrap();
	static ref ICON: String = {
		let raw = include_bytes!("../data/rain.ico");
		String::from_utf8_lossy(raw).to_string()
	};
}

static CMD: AtomicI32 = AtomicI32::new(-1);

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

/// Setups the application dir and config
pub fn setup() -> Result<(Config, PathBuf)> {
	let home: PathBuf = HOME.join(".noise");

	// create the dir
	fs::create_dir_all(home.clone())?;

	let config = home.join("config.toml");

	// get config
	let cfg = match File::open(config.clone()) {
		Ok(_) => Config::from_file(config.clone()),
		Err(_) => Config::create(config.clone()),
	}?;

	// create assets
	let icon_path = home.clone().join("icon.ico");
	match File::open(icon_path.clone()) {
		Ok(_) => {}
		Err(_) => {
			// Icon does not exist
			let raw_icon = include_bytes!("../data/rain.ico");
			let mut file = File::create(icon_path.clone())?;
			file.write_all(raw_icon)?;
			file.sync_all()?;
		}
	}

	Ok((cfg, icon_path))
}
