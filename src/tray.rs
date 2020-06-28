use crate::audio::set_cmd;
use anyhow::*;
use std::path::PathBuf;
use systray::{Application, Error};

/// Creates the system tray application
pub fn create(icon: PathBuf) -> Result<Application> {
	let mut app = Application::new()?;
	app.set_icon_from_file(&icon.to_string_lossy())?;

	app.add_menu_item("Start", |_| {
		set_cmd(1);
		Ok::<_, Error>(())
	})?;

	app.add_menu_item("Toggle", |_| {
		set_cmd(-1);
		Ok::<_, Error>(())
	})?;

	app.add_menu_separator()?;
	app.add_menu_item("Volume: 50%", |_| {
		set_cmd(50);
		Ok::<_, Error>(())
	})?;
	app.add_menu_item("Volume: 100%", |_| {
		set_cmd(100);
		Ok::<_, Error>(())
	})?;
	app.add_menu_item("Volume: 200%", |_| {
		set_cmd(200);
		Ok::<_, Error>(())
	})?;
	app.add_menu_separator()?;

	app.add_menu_item("Quit", |w| {
		w.quit();
		Ok::<_, Error>(())
	})?;

	Ok(app)
}
