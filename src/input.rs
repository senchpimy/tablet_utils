use chrono::{DateTime, Utc};
#[derive(Debug)]
pub struct StylusInputRaw {
    tv_sec: i64,
    tv_usec: i64,
    type_: u16,
    code: u16,
    val: i32,
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

type Position = i32;
type Pressed = bool;

#[derive(Debug)]
enum StylusData {
    X(Position),
    Y(Position),
    Btn1(Pressed),
    Btn2(Pressed),
}

#[derive(Debug)]
pub struct StylusInput {
    date: DateTime<Utc>,
    data: Option<StylusData>,
}

impl StylusInput {
    pub fn from_raw(raw: StylusInputRaw) -> Option<Self> {
        let timestamp = raw.tv_sec + (raw.tv_usec / 1000000);
        let date = DateTime::from_timestamp(timestamp, 0).unwrap();
        let data = match raw.type_ {
            1 => match raw.code {
                320 => Some(StylusData::Btn1(raw.val >= 1)),
                331 => Some(StylusData::Btn2(raw.val >= 1)),
                _ => None,
            },
            3 => match raw.code {
                0 => Some(StylusData::X(raw.val)),
                1 => Some(StylusData::Y(raw.val)),
                _ => None,
            },
            _ => None,
        };
        data.as_ref()?;
        Some(Self { date, data })
    }
}
