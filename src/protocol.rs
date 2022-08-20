#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, int_enum::IntEnum)]
pub enum Direction {
    UPLOAD = 0,
    DOWNLOAD = 1,
}

pub const PAYLOAD: u8 = b'a';
