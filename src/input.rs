use chrono::{DateTime, Utc};
#[derive(Debug)]
pub struct StylusInputRaw {
    tv_sec: i64,
    tv_usec: i64,
    type_: u16,
    code: u16,
    val: i32,
}

#[derive(Debug)]
pub struct EventHolder<T> {
    max_size: usize,
    elements: Vec<T>,
}

impl<T> EventHolder<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            elements: Vec::with_capacity(max_size),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.elements.len() == self.max_size {
            self.elements.remove(0); // Remove the oldest element
        }
        self.elements.push(item);
    }

    //pub fn to_slice(&self) -> &[T] {
    //    &self.elements
    //}

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    //pub fn is_empty(self) -> bool {
    //    self.elements.is_empty()
    //}
    //pub fn get_mut(&mut self, index: usize) -> &mut T {
    //    &mut self.elements[index]
    //}

    pub fn get_ref(&self, index: usize) -> &T {
        &self.elements[index]
    }

    pub fn last(&self) -> &T {
        &self.elements[self.elements.len() - 1]
    }

    pub fn last_mut(&mut self) -> &mut T {
        let len = self.elements.len();
        &mut self.elements[len - 1]
    }
}

pub fn parse_stylus_input(raw_data: &Vec<u8>, size: usize) -> Option<StylusInputRaw> {
    if raw_data.len() != size {
        return None; // Ensure the input data has the correct length
    }

    Some(StylusInputRaw {
        tv_sec: i64::from_ne_bytes(raw_data[0..8].try_into().unwrap()),
        tv_usec: i64::from_ne_bytes(raw_data[8..16].try_into().unwrap()),
        type_: u16::from_ne_bytes(raw_data[16..18].try_into().unwrap()),
        code: u16::from_ne_bytes(raw_data[18..20].try_into().unwrap()),
        val: i32::from_ne_bytes(raw_data[20..24].try_into().unwrap()),
    })
}

type Pressed = bool;

#[derive(Debug, PartialEq)]
pub enum StylusAction {
    Tilt(StylusCoord),
    Btn1(Pressed),
    Btn2(Pressed),
}

#[derive(Debug, PartialEq)]
pub enum StylusCoord {
    X(i32),
    Y(i32),
}

#[derive(Debug, PartialEq)]
pub enum StylusData {
    Coord(StylusCoord),
    Action(StylusAction),
    Pression, // I dont about this one tho
    Terminator,
}

#[derive(Debug)]
pub struct StylusInput {
    pub date: DateTime<Utc>,
    pub data: StylusData,
}

impl StylusInput {
    pub fn from_raw(raw: StylusInputRaw) -> Option<Self> {
        let timestamp = raw.tv_sec;
        let nanos = (raw.tv_usec * 1_000) as u32;
        let date = DateTime::from_timestamp(timestamp, nanos).unwrap();
        let data = match raw.type_ {
            0 => Some(StylusData::Terminator),
            1 => match raw.code {
                320 => Some(StylusData::Action(StylusAction::Btn1(raw.val >= 1))),
                331 => Some(StylusData::Action(StylusAction::Btn2(raw.val >= 1))),
                26 => {
                    println!("Tilt X");
                    Some(StylusData::Action(StylusAction::Tilt(StylusCoord::X(
                        raw.val,
                    ))))
                }
                27 => {
                    println!("Tilt Y");
                    Some(StylusData::Action(StylusAction::Tilt(StylusCoord::Y(
                        raw.val,
                    ))))
                }
                330 => {
                    //I dont know but I sometimes get this code
                    None
                }
                _ => None,
            },
            3 => match raw.code {
                0 => Some(StylusData::Coord(StylusCoord::X(raw.val))),
                1 => Some(StylusData::Coord(StylusCoord::Y(raw.val))),
                _ => None,
            },
            4 => Some(StylusData::Pression),
            _ => None,
        };
        data.map(|data| Self { date, data })
    }
}

#[derive(Debug)]
pub struct StylusButtonAction {
    pub x: i32,
    pub y: i32,
    pub action: StylusInput,
}
