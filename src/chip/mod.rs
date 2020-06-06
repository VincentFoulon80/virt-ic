use super::State;
pub mod gates;
pub mod generators;
pub mod memory;
pub mod cpu;
pub mod clocks;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PinType {
    Undefined,
    Input,
    Output,
    // Both can cause issues on Trace::communicate()
}

#[derive(Debug)]
pub struct Pin {
    pub number: u8,
    pub pin_type: PinType,
    pub state: State,
}
impl Pin {
    pub fn new(number: u8, pin_type: PinType) -> Pin {
        Pin {
            number,
            pin_type,
            state: State::Undefined
        }
    }
}

pub trait Chip: std::fmt::Debug {
    fn run(&mut self, elapsed_time: std::time::Duration);
    fn get_pin_qty(&self) -> u8;
    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str>;
    // fn set_pin(&mut self, pin: u8, state: &State);
}