use crate::{config::Config, tray};
use anyhow::Result;
use std::{fs::File, path::PathBuf};
use systray::Application;

pub struct Noise {
	pub tray: Application,
	pub song: Option<PathBuf>,
	pub cmd: i32,
	pub running: bool,
}

impl Noise {
	pub fn new() -> Result<Self> {
		let (cfg, icon) = crate::setup()?;

		let song = {
			let song_path = crate::HOME.join(".noise").join(cfg.sound.clone());
			match File::open(song_path.clone()) {
				Ok(_) => Some(song_path),
				Err(_) => None,
			}
		};

		Ok(Self {
			tray: tray::create(icon)?,
			song,
			cmd: 0,
			running: false,
		})
	}

	pub fn start(&mut self) -> Result<()> {
		crate::audio::start_audio(self.song.clone());
		self.tray.wait_for_message()?;

		Ok(())
	}
}
