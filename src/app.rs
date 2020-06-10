use crate::{config::Config, tray};
use anyhow::Result;
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};
use systray::Application;

pub struct Noise {
	pub tray: Application,
	pub song: Option<PathBuf>,
}

impl Noise {
	pub fn new() -> Result<Self> {
		let home = crate::home().join(".noise");
		let (cfg, icon) = Noise::setup(home.clone())?;

		let song = {
			let song_path = home.join(cfg.sound.clone());
			println!("{:?}", song_path);
			match File::open(song_path.clone()) {
				Ok(_) => Some(song_path),
				Err(_) => None,
			}
		};

		Ok(Self {
			tray: tray::create(icon)?,
			song,
		})
	}

	pub fn start(&mut self) -> Result<()> {
		crate::audio::start_audio(self.song.clone());
		self.tray.wait_for_message()?;

		Ok(())
	}

	/// Setups the application dir and config
	pub fn setup(home: PathBuf) -> Result<(Config, PathBuf)> {
		// create the dir
		fs::create_dir_all(home.clone())?;

		// get config
		let config = home.join("config.toml");
		let cfg = match File::open(config.clone()) {
			Ok(_) => Config::from_file(config.clone()),
			Err(_) => Config::create(config.clone()),
		}?;

		// create assets
		let icon_path = home.join("icon.ico");
		match File::open(icon_path.clone()) {
			Ok(_) => {}
			Err(_) => {
				// Icon does not exist
				let raw_icon = include_bytes!("../data/icon.ico");
				let mut file = File::create(icon_path.clone())?;
				file.write_all(raw_icon)?;
				file.sync_all()?;
			}
		}

		Ok((cfg, icon_path))
	}
}
