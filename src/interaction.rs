use crate::input::{self, StylusButtonAction};
use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
struct BtnEvent {
    pressed: StylusButtonAction,
    released: Option<StylusButtonAction>,
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
    latest_x: i32,
    latest_y: i32,
    last: LastAction,
}

impl State {
    pub fn new() -> Self {
        Self {
            btn1_events: input::EventHolder::new(5),
            btn2_events: input::EventHolder::new(5),
            latest_x: 0,
            latest_y: 0,
            last: LastAction::None,
        }
    }

    pub fn process(&mut self, data: input::StylusInput) {
        match &data.data {
            input::StylusData::Coord(val) => match val {
                input::StylusCoord::X(x) => {
                    if self.latest_x == 0 {
                        match self.last {
                            LastAction::Btn1 => {
                                let prim = self.btn1_events.last_mut();
                                prim.pressed.x = *x;
                            }
                            LastAction::Btn2 => {
                                let prim = self.btn2_events.last_mut();
                                prim.pressed.x = *x;
                            }
                            LastAction::None => {}
                        }
                    }
                    self.latest_x = *x;
                }
                input::StylusCoord::Y(y) => {
                    if self.latest_y == 0 {
                        match self.last {
                            LastAction::Btn1 => {
                                let prim = self.btn1_events.last_mut();
                                prim.pressed.y = *y;
                            }
                            LastAction::Btn2 => {
                                let prim = self.btn2_events.last_mut();
                                prim.pressed.y = *y;
                            }
                            LastAction::None => {}
                        }
                    }
                    self.latest_y = *y
                }
            },
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

        let events: &mut input::EventHolder<BtnEvent> = if events {
            &mut self.btn1_events
        } else {
            &mut self.btn2_events
        };

        if pressed {
            let item = BtnEvent {
                pressed: btn_event,
                released: None,
            };

            events.push(item);
        } else {
            let first = events.last_mut();
            first.released = Some(btn_event);
            self.latest_x = 0;
            self.latest_y = 0;
        }
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
