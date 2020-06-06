pub mod chip;
mod trace;
mod board;
mod socket;
pub use chip::{Chip, Pin, PinType};
pub use board::Board;
pub use trace::Trace;
pub use socket::Socket;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State {
    Undefined,
    High,
    Low
}

impl From<bool> for State {
    fn from(bit: bool) -> Self {
        if bit {
            State::High
        } else {
            State::Low
        }
    }
}
impl State {
    pub fn from_u8(data: u8, position: usize) -> Self {
        let bit = (data >> position) & 1;
        if bit == 1 {
            State::High
        } else {
            State::Low
        }
    }
    pub fn from_u16(data: u16, position: usize) -> Self {
        let bit = (data >> position) & 1;
        if bit == 1 {
            State::High
        } else {
            State::Low
        }
    }
    pub fn from_u32(data: u32, position: usize) -> Self {
        let bit = (data >> position) & 1;
        if bit == 1 {
            State::High
        } else {
            State::Low
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            State::High => true,
            _ => false
        }
    }
}