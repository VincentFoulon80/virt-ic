mod board;
pub mod chip;
mod save;
mod socket;
mod trace;
pub use board::Board;
pub use chip::{Chip, ChipInfo, Pin, PinType};
use serde::{Deserialize, Serialize};
pub use socket::Socket;
pub use trace::Trace;

/// Current's State
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum State {
    Undefined,
    High,
    Low,
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
            _ => false,
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            State::High => 1,
            _ => 0,
        }
    }
}
