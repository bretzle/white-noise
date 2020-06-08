use crate::{audio::start_audio, CMD};
use anyhow::*;
use std::{path::PathBuf, sync::atomic::Ordering};
use systray::{Application, Error};

/// Creates the system tray application
pub fn create(icon: PathBuf) -> Result<Application> {
	let mut app = Application::new()?;
	app.set_icon_from_file(&icon.to_string_lossy())?;

	app.add_menu_item("Start", |_| {
		CMD.store(1, Ordering::SeqCst);
		Ok::<_, Error>(())
	})?;

	app.add_menu_item("Toggle", |_| {
		CMD.store(-1, Ordering::SeqCst);
		Ok::<_, Error>(())
	})?;

	app.add_menu_separator()?;
	app.add_menu_item("Volume: 50%", |_| {
		CMD.store(50, Ordering::SeqCst);
		Ok::<_, Error>(())
	})?;
	app.add_menu_item("Volume: 100%", |_| {
		CMD.store(100, Ordering::SeqCst);
		Ok::<_, Error>(())
	})?;
	app.add_menu_item("Volume: 200%", |_| {
		CMD.store(200, Ordering::SeqCst);
		Ok::<_, Error>(())
	})?;
	app.add_menu_separator()?;

	app.add_menu_item("Quit", |w| {
		w.quit();
		Ok::<_, Error>(())
	})?;

	Ok(app)
}
