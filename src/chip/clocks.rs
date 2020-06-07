//! Clocks that pulse at different speeds
use super::super::State;
use super::{Pin, PinType, Chip};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

/// A 100 Hz simple clock
/// CLK: clock
/// ```
///        --------
///  CLK --|1    4|-- VCC
///  GND --|2    3|-- UNUSED
///        --------
/// ```
#[derive(Debug)]
pub struct Clock100Hz {
    pin: [Rc<RefCell<Pin>>; 4],
    timer: Duration,
    active: bool
}
impl Default for Clock100Hz {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock100Hz {
    pub const CLK: u8 = 1;
    pub const VCC: u8 = 4;
    pub const GND: u8 = 2;

    pub fn new() -> Self {
        Clock100Hz {
            pin: [
                Rc::new(RefCell::new(Pin::new(1, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(3, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(4, PinType::Input))),
            ],
            timer: Duration::new(0,0),
            active: false,
        }
    }
}
impl Chip for Clock100Hz {
    fn get_pin_qty(&self) -> u8 { 
        4
    }

    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str> { 
        if pin > 0 && pin <= 4 {
            Ok(self.pin[pin as usize-1].clone())
        } else {
            Err("Pin out of bounds")
        }
    }
    fn run(&mut self, time_elapsed: std::time::Duration) {
        if self.active {
            self.active = false;
            self.pin[0].borrow_mut().state = State::Low;
        } 
        // check alimented
        static LIMIT: Duration = Duration::from_millis(10);
        if self.pin[1].borrow().state == State::Low && self.pin[3].borrow().state == State::High {
            self.timer += time_elapsed;
            if self.timer > LIMIT {
                while self.timer > LIMIT {
                    self.timer -= LIMIT;
                }
                self.active = true;
                self.pin[0].borrow_mut().state = State::High;
            }
        } else {
            self.timer = Duration::new(0,0);
        }
    }
}