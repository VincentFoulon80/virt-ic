//! Generators that provide fixed currents
use super::super::State;
use super::{Pin, PinType, Chip};
use std::cell::RefCell;
use std::rc::Rc;

/// # A simple generator providing VCC and GND
/// 
/// # Diagram
/// ```
///        --------
///  VCC --|1    2|-- GND
///        --------
/// ```
#[derive(Debug)]
pub struct Generator {
    pin: [Rc<RefCell<Pin>>; 2],
}
impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator {
    pub const VCC: u8 = 1;
    pub const GND: u8 = 2;
    
    pub fn new() -> Self {
        let gen = Generator {
            pin: [
                Rc::new(RefCell::new(Pin::new(1, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(2, PinType::Output))),
            ]
        };
        gen.pin[0].borrow_mut().state = State::High;
        gen.pin[1].borrow_mut().state = State::Low;
        gen
    }
}
impl Chip for Generator {
    fn get_pin_qty(&self) -> u8 { 
        2
    }

    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str> { 
        if pin > 0 && pin <= 2 {
            Ok(self.pin[pin as usize-1].clone())
        } else {
            Err("Pin out of bounds")
        }
    }
    fn run(&mut self, _: std::time::Duration) {
        self.pin[0].borrow_mut().state = State::High;
        self.pin[1].borrow_mut().state = State::Low;
    }
}