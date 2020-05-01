#[macro_use]
extern crate lazy_static;

use rodio::{Decoder, Sink};
use std::{
    env, fs,
    fs::File,
    io::{BufReader, ErrorKind, Write},
    path::PathBuf,
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
    thread,
    time::Duration,
};
use systray::{Application, Error};

static SINGLE: AtomicBool = AtomicBool::new(false);
static CMD: AtomicI32 = AtomicI32::new(0);

const RAW_RAIN: &'static [u8] = include_bytes!("../data/rain.mp3");
const RAW_ICON: &'static [u8] = include_bytes!("../data/rain.ico");

lazy_static! {
    static ref NOISE_PATH: String = {
        let dir = env::temp_dir().join("noise");
        match fs::create_dir(&dir) {
            Ok(_) => {}
            Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {}
            Err(e) => println!("{:?}", e),
        }
        let fp: PathBuf = dir.join("rain.mp3");

        let mut f = File::create(&fp).expect("Couldn't create file");
        f.write_all(RAW_RAIN).expect("Couldn't write to rain.mp3");
        f.sync_all().expect("Couldn't sync rain.mp3");

        fp.as_path().to_str().unwrap().to_string()
    };
    static ref ICON: String = {
        let fp = env::temp_dir().join("noise").join("rain.ico");
        let mut f = File::create(&fp).unwrap();
        f.write_all(RAW_ICON).unwrap();
        f.sync_all().unwrap();

        fp.as_path().to_str().unwrap().to_string()
    };
}

fn main() -> Result<(), Error> {
    let mut app = Application::new()?;
    let a = &ICON;
    app.set_icon_from_file(a)?;

    app.add_menu_item("Start", |_| {
        start_audio();
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

    app.wait_for_message()?;

    Ok(())
}

fn start_audio() {
    if !SINGLE.load(Ordering::SeqCst) {
        thread::spawn(|| {
            let device = rodio::default_output_device().unwrap();
            let sink = Sink::new(&device);

            loop {
                if sink.empty() {
                    let p: &str = &NOISE_PATH;
                    sink.append(Decoder::new(BufReader::new(File::open(p).unwrap())).unwrap());
                    println!("Added song to sink");
                }
                match CMD.load(Ordering::Relaxed) {
                    0 => {}
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
        SINGLE.swap(true, Ordering::Relaxed);
    }
}
