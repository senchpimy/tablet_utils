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
    let r = false;
    if r != false {
        println!("");
    }
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
    let r = Command::new("brillo")
        .output()
        .expect("failed to execute process");
    let string = &r.stdout[..2];
    let string_representation = String::from_utf8(string.to_vec()).unwrap();
    println!("{}", string_representation);
}

fn rundaemon() {
    let event_size = mem::size_of::<input::StylusInputRaw>();

    // Crear un closure para leer y procesar eventos
    let read_and_process = |event_device: &'static str| {
        thread::spawn(move || {
            // Abrir el dispositivo
            let mut f = File::open(event_device)
                .expect(&format!("Failed to open input device: {}", event_device));
            let mut buffer = vec![0u8; event_size];
            let mut state = interaction::State::new();

            println!("Started Reading from {}", event_device);
            loop {
                // Intentar leer del dispositivo
                if f.read_exact(&mut buffer).is_ok() {
                    let r = input::parse_stylus_input(&buffer, event_size);
                    if let Some(raw) = r {
                        let data = input::StylusInput::from_raw(raw);
                        if let Some(data) = data {
                            state.process(data);
                            state.handle_live();
                        }
                    }
                } else {
                    eprintln!("Incomplete event on {}", event_device);
                }
            }
        })
    };

    // Iniciar los hilos para ambos dispositivos
    let handle1 = read_and_process("/dev/input/event12");
    let handle2 = read_and_process("/dev/input/event13");

    // Esperar a que ambos hilos terminen (nunca sucederá en este caso)
    handle1.join().unwrap();
    handle2.join().unwrap();
}
