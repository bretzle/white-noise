use crate::{app::Details, CMD};
use rodio::{Decoder, Sink};
use std::{fs::File, io::BufReader, sync::atomic::Ordering, thread, time::Duration};

pub fn start_audio(details: Details) {
	thread::spawn(|| {
		if details.song.is_none() {
			println!("You currently have no audio file setup.");
			println!("You must add a path to a file in the config");
			println!("{:?}", crate::HOME.join(".noise").join("config.toml"));

			std::process::exit(0);
		}

		let device = rodio::default_output_device().unwrap();
		let sink = Sink::new(&device);
		let file = File::open(details.song.unwrap()).unwrap();

		loop {
			if sink.empty() {
				let reader = BufReader::new(file.try_clone().unwrap());
				let decoder = Decoder::new(reader).unwrap();
				sink.append(decoder);
				println!("Added song to sink");
			}
			match CMD.load(Ordering::Relaxed) {
				0 => {}
				1 => {
					CMD.store(0, Ordering::SeqCst);
					sink.play();
				}
				-1 => {
					CMD.store(0, Ordering::SeqCst);
					match sink.is_paused() {
						true => sink.play(),
						false => sink.pause(),
					}
				}
				50 => {
					CMD.store(0, Ordering::SeqCst);
					sink.set_volume(0.5);
				}
				100 => {
					CMD.store(0, Ordering::SeqCst);
					sink.set_volume(1.0);
				}
				200 => {
					CMD.store(0, Ordering::SeqCst);
					sink.set_volume(2.0);
				}
				_ => println!("Unknown command: {}", CMD.load(Ordering::Relaxed)),
			}
			thread::sleep(Duration::from_millis(250));
		}
	});
}
