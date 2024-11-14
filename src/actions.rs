use crate::{
    input::EventHolder,
    interaction::{ActionType, BtnEvent},
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LineDirection {
    BottomUp,
    UpBottom,
    LeftRigth,
    RigthLeft,
    None,
}

const MAX_DIFFERENCE_COORDS: i32 = 1000;
const MIN_TIME_POINT: i64 = 300;
const MAX_TIME_LINE: i64 = 600;
const SCREEN_HEIGTH: i32 = 20000;
const SCREEN_WIDTH: i32 = 35000;

pub fn match_interaction(interaction: &BtnEvent, _path: &mut Vec<(i32, i32)>) -> ActionType {
    let pressed = &interaction.pressed;
    if let Some(released) = interaction.released.as_ref() {
        let time_of_event = released.action.date - interaction.pressed.action.date;
        let milis = time_of_event.num_milliseconds();
        dbg!(released.x);
        dbg!(pressed.x);
        let diff_x = (released.x - pressed.x).abs();
        let diff_y = (released.y - pressed.y).abs();
        if milis < MIN_TIME_POINT
            && diff_x < MAX_DIFFERENCE_COORDS
            && diff_y < MAX_DIFFERENCE_COORDS
        {
            println!("Punto");
            return ActionType::Point;
        } else {
            //Mejorar sistema para identificar lineas rectas
            let distancia = ((released.x as f64 - pressed.x as f64).powi(2)
                + (released.y as f64 - pressed.y as f64).powi(2))
            .sqrt();
            let line_dir = if distancia > 1000. && milis < MAX_TIME_LINE {
                if diff_y < SCREEN_WIDTH / 10 && (300..1000).contains(&milis) {
                    if pressed.x > released.x {
                        //derecha a izq
                        LineDirection::RigthLeft
                    } else {
                        LineDirection::LeftRigth
                        //izq a dere
                    }
                } else if diff_x < SCREEN_HEIGTH / 7 && (300..1000).contains(&milis) {
                    if pressed.y > released.y {
                        LineDirection::BottomUp
                        //abajo -> arriba
                    } else {
                        LineDirection::UpBottom
                        //arriba -> abajo
                    }
                } else {
                    LineDirection::None
                }
            } else {
                LineDirection::None
            };
            if line_dir != LineDirection::None {
                println!("Linea recta milis: {} X: {} Y: {}", milis, diff_x, diff_y);
                dbg!(&line_dir);
                return ActionType::StraigthLine(line_dir);
            }
            println!(
                "milis: {} X: {} Y: {} scre_wi {}, scre_h {}",
                milis,
                diff_x,
                diff_y,
                SCREEN_WIDTH / 3,
                SCREEN_HEIGTH / 3
            );
        }
    }
    ActionType::Line
}

pub enum Actions {
    ChangeWallpaper, //Dumbass me
    ChangeWorkspace(LineDirection),
    None,
}

pub fn match_interactions(vec: &mut EventHolder<BtnEvent>, btn1: bool) -> Actions {
    if !btn1 {
        return Actions::None;
    }
    let len = vec.len();
    if len >= 2 {
        let b1 = vec.get_ref(len - 1);
        let b2 = vec.get_ref(len - 2);
        if let Some(type_) = &b2.type_ {
            if let ActionType::Point = type_ {
                if let Some(type_) = &b1.type_ {
                    if let ActionType::Point = type_ {
                        let t1 = b1.released.as_ref().unwrap().action.date;
                        let t2 = b2.pressed.action.date;
                        let diff = t2 - t1;
                        if diff.num_milliseconds() < MAX_TIME_LINE {
                            dbg!("Elimndaod");
                            //Eliminamos los ultimos dos elementos tempralmente
                            //para evitar que se dispare imnediatamente despues de un evento
                            //pero lo ideal seria simplemente ignorarlos una vez que se realizo un
                            //match
                            //if vec.len() >= 2 {
                            //    vec.pop();
                            //    vec.pop();
                            //}
                            return Actions::ChangeWallpaper;
                        }
                    }
                }
            }
        }
    }
    if let Some(dir) = &vec.last().type_ {
        match dir {
            ActionType::StraigthLine(line) => {
                return Actions::ChangeWorkspace((*line).clone());
            }
            _ => {}
        }
    };
    Actions::None
}
