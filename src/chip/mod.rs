//! Chip trait, Pins and premade Chips
use super::State;
pub mod gates;
pub mod generators;
pub mod memory;
pub mod cpu;
pub mod clocks;
use std::cell::RefCell;
use std::rc::Rc;

/// The type of a Pin, that can be Input or Output
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PinType {
    Undefined,
    Input,
    Output,
    // Both // removed because it can cause issues on Trace::communicate(). It's better to swap the pin when needed
}

/// A chip's Pin. Can be of type Input or Output, and holds a State
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

/// Chip : a trait that represents chips on board
pub trait Chip: std::fmt::Debug {
    /// Runs the chip for a certain amount of time
    fn run(&mut self, elapsed_time: std::time::Duration);
    /// Returns the number of pins the chip has
    fn get_pin_qty(&self) -> u8;
    /// Get a pin of the chip
    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str>;
    /// Get the state of the specified Pin
    fn get_pin_state(&mut self, pin: u8) -> State {
        if let Ok(pin) = self.get_pin(pin) {
            pin.borrow().state.clone()
        } else {
            State::Undefined
        }
    }
    /// Set the state of the specified Pin
    fn set_pin_state(&mut self, pin: u8, state: &State) {
        if let Ok(pin) = self.get_pin(pin) {
            pin.borrow_mut().state = state.clone();
        } 
    }
}