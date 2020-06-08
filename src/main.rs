// #![windows_subsystem = "windows"]

#[macro_use]
extern crate lazy_static;

mod audio;
mod config;
mod tray;

use anyhow::*;
use config::Config;
use std::{
	fs::File,
	fs::{self, read_to_string},
	io::Write,
	path::PathBuf,
	sync::atomic::{AtomicI32, Ordering},
};
use systray::{Application, Error};

lazy_static! {
	static ref HOME: PathBuf = home::home_dir().unwrap();
	static ref ICON: String = {
		let raw = include_bytes!("../data/rain.ico");
		String::from_utf8_lossy(raw).to_string()
	};
}

static CMD: AtomicI32 = AtomicI32::new(0);

fn main() {
	if let Err(e) = run() {
		eprintln!("{:?}", e);
	}
}

fn run() -> Result<()> {
	let (cfg, icon) = setup()?;
	let mut app = tray::create(icon)?;

	app.wait_for_message()?;

	Ok(())
}

/// Setups the application dir and config
fn setup() -> Result<(Config, PathBuf)> {
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
