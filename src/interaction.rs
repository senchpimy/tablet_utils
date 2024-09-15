use std::time::Instant;

use std::process::Command;

use crate::actions::{self, Actions, LineDirection};
use crate::input::{self, StylusAction, StylusButtonAction, StylusData, StylusInput};
use crate::{set_brillo, set_volume};
use once_cell::sync::Lazy;
use std::env;

#[derive(Debug, Eq, PartialEq)]
pub enum ActionType {
    Point,
    Line,
    StraigthLine(LineDirection),
}

#[derive(Debug)]
pub struct BtnEvent {
    pub pressed: StylusButtonAction,
    pub released: Option<StylusButtonAction>,
    pub type_: Option<ActionType>,
}

#[derive(Debug)]
enum LastAction {
    Btn1,
    Btn2,
    None,
}

pub struct State {
    btn1_events: input::EventHolder<BtnEvent>,
    btn2_events: input::EventHolder<BtnEvent>,
    btn1_path: Vec<(i32, i32)>,
    btn2_path: Vec<(i32, i32)>,
    pub latest_x: i32,
    pub latest_y: i32,
    latest_x_pushed: i32,
    latest_y_pushed: i32,
    last: LastAction,
    latest_data_saved: Instant,
    pression_status: bool,
    pub btn1_pressed: bool,
    pub btn2_pressed: bool,
    btn2_status: bool,
}

fn map_value(input: u32) -> u32 {
    (100 - (input - 2000) * 99 / 16000).clamp(1, 100)
}

fn gui(ventana: bool, abrir: bool) {
    //eww -c .config/eww/brigth open my-window
    let ventana = if ventana { "brigth" } else { "vol" };
    let abrir = if abrir { "open" } else { "close" };
    let script_path = format!("{}/.config/eww/{}", *HOME, ventana);
    dbg!(abrir);
    let output = Command::new("eww")
        .arg("-c")
        .arg(script_path)
        .arg(abrir)
        .arg("my-window")
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        // Convierte la salida de bytes a string y la imprime
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Salida del comando:\n{}", stdout);
    } else {
        // En caso de error, muestra el error en stderr
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error al ejecutar el comando:\n{}", stderr);
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            btn1_events: input::EventHolder::new(5),
            btn2_events: input::EventHolder::new(5),
            latest_x: 0,
            latest_y: 0,
            latest_x_pushed: 0,
            latest_y_pushed: 0,
            btn1_path: Vec::new(),
            btn2_path: Vec::new(),
            last: LastAction::None,
            latest_data_saved: Instant::now(),
            pression_status: false,
            btn1_pressed: false,
            btn2_pressed: false,
            btn2_status: false,
        }
    }

    pub fn handle_live(&mut self) {
        if (3_000..17_000).contains(&self.latest_y) {
            let val = map_value(self.latest_y as u32);
            if (31_000..35_000).contains(&self.latest_x) {
                //volumen
                set_volume(val);
            } else if (0..4_000).contains(&self.latest_x) {
                //brillo
                set_brillo(val);
            }
        }
    }

    pub fn process(&mut self, data: input::StylusInput) {
        match &data.data {
            input::StylusData::Coord(val) => {
                match val {
                    input::StylusCoord::X(x) => {
                        if self.latest_x == 0 {
                            if let Some(prim) = match self.last {
                                LastAction::Btn1 => Some(self.btn1_events.last_mut()),
                                LastAction::Btn2 => Some(self.btn2_events.last_mut()),
                                LastAction::None => None,
                            } {
                                prim.pressed.x = *x;
                            }
                        }
                        self.latest_x = *x;
                    }
                    input::StylusCoord::Y(y) => {
                        if self.latest_y == 0 {
                            if let Some(prim) = match self.last {
                                LastAction::Btn1 => Some(self.btn1_events.last_mut()),
                                LastAction::Btn2 => Some(self.btn2_events.last_mut()),
                                LastAction::None => None,
                            } {
                                prim.pressed.y = *y;
                            }
                        }
                        self.latest_y = *y;
                    }
                }
                let path = &mut self.btn1_path;
                let time_diff = self.latest_data_saved.elapsed().as_millis();
                let diff_x = (self.latest_x - self.latest_x_pushed).abs();
                let diff_y = (self.latest_y - self.latest_y_pushed).abs();
                let diff_coord = 100;
                if time_diff >= 20
                    && self.latest_x > 0
                    && self.latest_y > 0
                    && (diff_y >= diff_coord || diff_x >= diff_coord)
                {
                    self.latest_data_saved = Instant::now();
                    path.push((self.latest_x, self.latest_y));
                    self.latest_x_pushed = self.latest_x;
                    self.latest_y_pushed = self.latest_y;
                }
            }
            input::StylusData::Action(val) => match val {
                input::StylusAction::Tilt(_) => {}
                input::StylusAction::Btn1(val) => {
                    //ignore val
                    //let val = self.pression_status;
                    let r = self.handle_button_event(*val, true, data);
                    self.last = LastAction::Btn1;
                    match r {
                        Actions::None => {}
                        Actions::ChangeWallpaper => {
                            change_wallpaper();
                        }
                        Actions::ChangeWorkspace(dir) => match dir {
                            LineDirection::LeftRigth => left_rigth(),
                            LineDirection::RigthLeft => right_left(),
                            _ => {}
                        },
                    }
                }
                input::StylusAction::Btn2(val) => {
                    dbg!(val);
                    self.handle_button_event(*val, false, data);
                    self.last = LastAction::Btn2;
                }
            },
            input::StylusData::Pression => {
                self.pression_status = !self.pression_status;
            }
            input::StylusData::Terminator => {}
        }
    }

    fn handle_button_event(&mut self, pressed: bool, events: bool, data: StylusInput) -> Actions {
        if let StylusData::Action(StylusAction::Btn1(_) | StylusAction::Btn2(_)) = data.data {
        } else {
            return Actions::None;
        }

        let btn_event = StylusButtonAction {
            x: self.latest_x,
            y: self.latest_y,
            action: data,
        };

        let (events, pressed_stated, path, btn1) = if events {
            (
                &mut self.btn1_events,
                &mut self.btn1_pressed,
                &mut self.btn1_path,
                true,
            )
        } else {
            (
                &mut self.btn2_events,
                &mut self.btn2_pressed,
                &mut self.btn2_path,
                false,
            )
        };

        if pressed {
            let item = BtnEvent {
                pressed: btn_event,
                released: None,
                type_: None,
            };

            events.push(item);
            *pressed_stated = true;
        } else {
            let first = events.last_mut();
            first.released = Some(btn_event);
            self.latest_x = 0;
            self.latest_y = 0;
            let inter = actions::match_interaction(first, path);
            first.type_ = Some(inter);
            *path = Vec::new();
            *pressed_stated = false;
        }
        dbg!(&self.btn2_pressed);
        actions::match_interactions(events, btn1)
    }
}

static HOME: Lazy<String> =
    Lazy::new(|| env::var("HOME").expect("HOME environment variable not set"));

fn change_wallpaper() {
    let script_path = format!("{}/.local/share/bin/swwwallpaper.sh", *HOME);

    // Use sudo to run the script as a different user
    let _output = std::process::Command::new(script_path)
        .arg("-n") // Additional argument for the script
        .output()
        .expect("Failed to execute command");
}

fn left_rigth() {
    let _output = std::process::Command::new("hyprctl")
        .arg("dispatch")
        .arg("workspace")
        .arg("-1")
        .output()
        .expect("Failed to execute command");
}

fn right_left() {
    // hyprctl dispatch workspace +1
    let _output = std::process::Command::new("hyprctl")
        .arg("dispatch")
        .arg("workspace")
        .arg("+1")
        .output()
        .expect("Failed to execute command");
}
