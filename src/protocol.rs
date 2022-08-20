#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, int_enum::IntEnum)]
pub enum Direction {
    UPLOAD = 0,
    DOWNLOAD = 1,
}

pub const PAYLOAD_BUF_SIZE: u32 = 4096;
pub const PAYLOAD_BUF: [u8; PAYLOAD_BUF_SIZE as usize] =
    [b'a'; PAYLOAD_BUF_SIZE as usize];
