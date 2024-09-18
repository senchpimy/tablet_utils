use alsa::mixer::{Mixer, SelemId};
use std::sync::{Arc, Mutex};
use std::thread;

use clap::Parser;
use std::mem;
use std::process::Command;
use std::{fs::File, io::Read};

mod actions;
mod input;
mod interaction;

/// Get Custom functionality as a tablet in your device
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Get volumne of the system
    #[arg(short, long)]
    volume: bool,

    /// Set volumune using alsa
    #[arg(long)]
    set_volume: Option<u32>,

    /// Set brightness using brillo Command
    #[arg(long)]
    set_brillo: Option<u32>,

    /// Get brightness
    #[arg(short, long)]
    brillo: bool,

    /// Start daemon
    #[arg(short, long)]
    daemon: bool,
}

fn main() {
    let args = Args::parse();
    if args.daemon {
        rundaemon();
    } else if args.volume {
        get_volume();
    } else if args.brillo {
        get_brillo();
    } else if args.set_volume.is_some() {
        set_volume(args.set_volume.unwrap());
    } else if args.set_brillo.is_some() {
        set_brillo(args.set_brillo.unwrap());
    }
}

fn get_volume() {
    let mixer = Mixer::new("default", false).expect("Failed to open mixer");
    let selem_id = SelemId::new("Master", 0);
    let selem = mixer.find_selem(&selem_id).expect("Failed to find selem");
    let (_, max) = selem.get_playback_volume_range();
    let volume = selem
        .get_playback_volume(alsa::mixer::SelemChannelId::FrontLeft)
        .expect("Failed to get playback volume");
    let volume_percent = (volume as f64 / max as f64) * 100.0;
    print!("{}", volume_percent as i32);
}

pub fn set_volume(v: u32) {
    let actual_value: i64 = (65000 / 100) * v as i64;
    let mixer = Mixer::new("default", false).expect("Failed to open mixer");
    let selem_id = SelemId::new("Master", 0);
    let selem = mixer.find_selem(&selem_id).expect("Failed to find selem");
    match selem.set_playback_volume_all(actual_value) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn set_brillo(v: u32) {
    let _r = Command::new("brillo")
        .arg("-S")
        .arg(format!("{v}"))
        .output()
        .expect("failed to execute process");
}

fn get_brillo() {
    let _r = Command::new("brillo")
        .output()
        .expect("failed to execute process");
}

fn rundaemon() {
    let event_device = detect_event_device().unwrap(); //detectar automaticamente
    let event_size = mem::size_of::<input::StylusInputRaw>();
    let mut f = File::open(event_device).expect("Failed to open input device");
    println!("Started Reading");
    println!("AAA");
    let mut buffer = vec![0u8; event_size];
    let mut state = interaction::State::new();
    loop {
        if f.read_exact(&mut buffer).is_ok() {
            let r = input::parse_stylus_input(&buffer, event_size);
            if let Some(raw) = r {
                let data = input::StylusInput::from_raw(raw);
                if let Some(data) = data {
                    state.process(data);
                    state.handle_live();
                }
            }
            //println!("btn1 {}, btn2 {}", state.btn1_pressed, state.btn2_pressed);
        } else {
            eprintln!("incomplete event");
        }
    }
}
fn detect_event_device() -> Option<String> {
    let devices = vec!["/dev/input/event12", "/dev/input/event13"];
    let event_size = std::mem::size_of::<input::StylusInputRaw>();
    let detected_device = Arc::new(Mutex::new(None));

    let mut handles = vec![];

    for device in devices {
        let device = device.to_string();
        let detected_device = Arc::clone(&detected_device);

        let handle = thread::spawn(move || {
            if let Ok(mut f) = File::open(&device) {
                let mut buffer = vec![0u8; event_size];
                loop {
                    if f.read_exact(&mut buffer).is_ok() {
                        let mut detected = detected_device.lock().unwrap();
                        if detected.is_none() {
                            *detected = Some(device.clone());
                        }
                        break;
                    }
                }
            }
        });

        handles.push(handle);
    }

    // Esperar a que uno de los hilos detecte el dispositivo correcto
    for handle in handles {
        let _ = handle.join();
    }

    // Obtener el dispositivo detectado
    let detected_device = detected_device.lock().unwrap().clone();
    detected_device
}
