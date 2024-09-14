use alsa::mixer::{Mixer, SelemId};

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
    set_volume: Option<u64>,

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

fn set_volume(v: u64) {
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

fn set_brillo(v: u32) {
    let r = Command::new("brillo")
        .arg("-S")
        .arg(format!("{v}"))
        .output()
        .expect("failed to execute process");
    if !r.status.success() {
        eprintln!(
            "Error change brightness {}",
            String::from_utf8_lossy(&r.stderr)
        );
    }
}

fn get_brillo() {
    let r = Command::new("brillo")
        .output()
        .expect("failed to execute process");
    if !r.status.success() {
        eprintln!(
            "Error change brightness {}",
            String::from_utf8_lossy(&r.stderr)
        );
    }
}

fn rundaemon() {
    let event_device = "/dev/input/event13";
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
                }
            }
            //println!("btn1 {}, btn2 {}", state.btn1_pressed, state.btn2_pressed);
        } else {
            eprintln!("incomplete event");
        }
    }
}
