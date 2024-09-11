use std::{default, time::Instant};

use crate::actions::{self};
use crate::input::{self, StylusButtonAction};

#[derive(Debug)]
pub enum ActionType {
    Point,
    Line,
    StraigthLine(LineDirection),
}

#[derive(Debug)]
pub enum LineDirection {
    BottomUp,
    UpBottom,
    LeftRigth,
    RigthLeft,
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
    latest_x: i32,
    latest_y: i32,
    latest_x_pushed: i32,
    latest_y_pushed: i32,
    last: LastAction,
    latest_data_saved: Instant,
    interactions: input::EventHolder<ActionType>,
    btn1_pressed: bool,
    btn2_pressed: bool,
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
            interactions: input::EventHolder::new(10),
            btn1_pressed: false,
            btn2_pressed: false,
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
                    dbg!(path.len());
                    dbg!("Se supone q cada 20 milis y algo mas");
                }
                dbg!(time_diff);
            }
            input::StylusData::Action(val) => match val {
                input::StylusAction::Tilt(_) => todo!(),
                input::StylusAction::Btn1(val) => {
                    self.handle_button_event(*val, true, data);
                    self.last = LastAction::Btn1;
                }
                input::StylusAction::Btn2(val) => {
                    self.handle_button_event(*val, false, data);
                    self.last = LastAction::Btn2;
                }
            },
        }
        //self.print_button_events();
    }

    fn handle_button_event(&mut self, pressed: bool, events: bool, data: input::StylusInput) {
        let btn_event = StylusButtonAction {
            x: self.latest_x,
            y: self.latest_y,
            action: data,
        };

        let (events, pressed_stated, path) = if events {
            (
                &mut self.btn1_events,
                &mut self.btn1_pressed,
                &mut self.btn1_path,
            )
        } else {
            (
                &mut self.btn2_events,
                &mut self.btn2_pressed,
                &mut self.btn2_path,
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
        actions::match_interactions(events.to_slice());
    }

    pub fn print_button_events(&self) {
        match self.last {
            LastAction::Btn1 => {
                let button_name = "Button 1";
                let press_time = self.btn1_events.last().pressed.action.date;
                let press_coords = (
                    self.btn1_events.last().pressed.x,
                    self.btn1_events.last().pressed.y,
                );

                println!(
                    "{}: Pressed at {} at coordinates ({}, {})",
                    button_name, press_time, press_coords.0, press_coords.1
                );
                if let Some(release_event) = &self.btn1_events.last().released {
                    let release_time = release_event.action.date;
                    let release_coords = (release_event.x, release_event.y);
                    let duration = release_time - press_time;

                    println!(
                        "{}: Released at {} at coordinates ({}, {}) after {} seconds",
                        button_name,
                        release_time,
                        release_coords.0,
                        release_coords.1,
                        duration.num_seconds()
                    );
                    dbg!(&self.btn1_events);
                }
            }
            LastAction::Btn2 => todo!(),
            LastAction::None => {}
        }
        {}
    }
}
