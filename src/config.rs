use anyhow::Result;
use serde_derive::*;
use std::{
	fs::{read_to_string, File},
	io::Write,
	path::Path,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
	sound: String,
}

impl Config {
	pub fn create<T: AsRef<Path> + Clone>(path: T) -> Result<Self> {
		let mut file = File::create(path.clone())?;
		let string = toml::to_string(&Config::default())?;
		file.write_all(string.as_bytes())?;

		Self::from_file(path)
	}

	pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
		let string = read_to_string(path)?;
		let config: Config = toml::from_str(&string)?;

		Ok(config)
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			sound: "".to_owned(),
		}
	}
}
