pub mod chip;
mod trace;
mod board;
mod socket;
pub use chip::{Chip, Pin, PinType};
pub use board::Board;
pub use trace::Trace;
pub use socket::Socket;

#[derive(Debug, PartialEq, Clone)]
pub struct Current {
    pub voltage: f32,
    pub amperage: f32
}

/// Current's State
#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Undefined,
    High,
    Low,
    Analog(Current)
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
    pub fn from_current(current: Current) -> Self {
        State::Analog(current)
    }
    pub fn from_volt_amper(voltage: f32, amperage: f32) -> Self {
        State::Analog(Current {
            voltage,
            amperage
        })
    }

    pub fn as_bool(&self) -> bool {
        match self {
            State::High => true,
            State::Analog(_) => self.to_digital().as_bool(),
            _ => false
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            State::High => 1,
            State::Analog(_) => self.to_digital().as_u8(),
            _ => 0
        }
    }

    pub fn to_digital(&self) -> Self {
        match self {
            State::High | State::Low | State::Undefined => self.clone(),
            State::Analog(current) => {
                // TODO: arbitrary values here
                if current.voltage > 0.5 && current.amperage > 0.0 {
                    State::High
                } else if current.amperage > 0.0 {
                    State::Low
                } else {
                    State::Undefined
                }
            }
        }
    }

    pub fn to_analog(&self) -> Self {
        State::Analog(self.to_current())
    }

    pub fn to_current(&self) -> Current {
        // TODO: arbitrary values here
        match self {
            State::High => Current{
                voltage: 5.0,
                amperage: 1.0
            },
            State::Low => Current{
                voltage: 0.0,
                amperage: 1.0
            },
            State::Undefined => Current{
                voltage: 0.0,
                amperage: 0.0
            },
            State::Analog(current) => current.clone()
        }
    }
}