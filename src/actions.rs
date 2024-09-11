use crate::interaction::{ActionType, BtnEvent};

pub fn match_interaction(interaction: &BtnEvent, path: &mut Vec<(i32, i32)>) -> ActionType {
    let pressed = &interaction.pressed;
    if let Some(released) = interaction.released.as_ref() {
        let time_of_event = released.action.date - interaction.pressed.action.date;
        let milis = time_of_event.num_milliseconds();
        let diff_x = (released.x - pressed.x).abs();
        let diff_y = (released.y - pressed.y).abs();
        let diff_max = 500;
        if milis < 400 && diff_x < diff_max && diff_y < diff_max {
            println!("Punto");
            return ActionType::Point;
        } else {
            println!("milis: {} X: {} Y: {}", milis, diff_x, diff_y);
        }
    }
    //let mut pendientes: Vec<f32> = Vec::new();
    //for window in path.windows(2) {
    //    if let [(x1, y1), (x2, y2)] = window {
    //        if x1 != x2 {
    //            // Evitar división por cero
    //            let mut slope = (*y2 - *y1) as f32 / (*x2 - *x1) as f32;
    //            slope *= 100.;
    //            pendientes.push(slope);
    //        }
    //    }
    //}
    ////Eliminar el ruido
    //let len = pendientes.len();
    //let remover = ((len as f32 / 100.) * 20.) as usize;
    //let pendientes = pendientes[remover..len - (remover * 2)].to_vec();
    //dbg!("Elementos removidos");
    //dbg!(remover);
    //let desv = desviacion_estandar(&pendientes);
    //let media: f32 = pendientes.iter().sum::<f32>() / pendientes.len() as f32;
    //let resultado: Vec<(f32, f32)> = pendientes
    //    .iter()
    //    .map(|&valor| {
    //        let z_score = (valor - media) / desv;
    //        (valor, z_score)
    //    })
    //    .collect();
    //dbg!(len);
    //dbg!(&pendientes);
    //dbg!(resultado);
    //println!("AAAaaaA");
    //let pendientes: Vec<bool> = pendientes
    //    .windows(5)
    //    .map(|w| (w[0] - w[4]).abs() > 0.01)
    //    .collect();
    //dbg!(pendientes);
    //println!("AAAaaaA");
    //for window in pendientes.windows(2) {
    //    if let [p1, p2] = window {
    //        let diff = (p1 - p2).abs();
    //        if diff > 0.01 {
    //            dbg!(diff);
    //            println!("Linea no fue recta");
    //            //return ActionType::Line;
    //        } else {
    //            dbg!(diff);
    //            println!("Linea reacta")
    //        }
    //    }
    //}
    ActionType::Line
}

pub enum Actions {
    ChangeWallpaper, //Dumbass me
    None,
}

pub fn match_interactions(vec: &[BtnEvent]) -> Actions {
    let mut time_vec = Vec::new(); // Guardar para no estar creando
    println!("AAAAA");
    for item in vec.windows(2) {
        if let [b1, b2] = item {
            //matcheamos el segundo por que si matchea significa que ya matcheo el primero
            if let Some(type_) = &b2.type_ {
                match type_ {
                    ActionType::Point => {
                        if let Some(type_) = &b1.type_ {
                            if let ActionType::Point = type_ {
                                let t1 = b1.released.as_ref().unwrap().action.date;
                                //Como ya sabemos que b2 es un Point no hay necesidad de
                                //verificar la duracion de cuando se presiono
                                let t2 = b2.pressed.action.date;
                                let diff = t2 - t1;
                                time_vec.push(diff);
                            }
                        }
                    }
                    ActionType::Line => {}
                    ActionType::StraigthLine(_) => {}
                }
            }
        }
    }
    let mut num_point = 0;
    for delta in time_vec.iter().rev() {
        if delta.num_milliseconds() > 600 {
            break;
        }
        num_point += 1;
    }
    if num_point == 1 {
        return Actions::ChangeWallpaper;
    }
    Actions::None
}
