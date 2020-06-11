//! Buttons and other physically interactable chips
use super::{Chip, Pin, PinType};
use crate::State;
use std::cell::RefCell;
use std::rc::Rc;

/// # A simple button
/// Transmit the IN signal in the OUT pin when he is down
/// you'll need to use `press()` and `release()` to change its state
///
/// # Diagram
/// ```
///        --------
///  IN  --|1    2|-- OUT
///        --------
/// ```
#[derive(Debug)]
pub struct Button {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 2],
    down: bool,
}
impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Button {
    pub const IN: u8 = 1;
    pub const OUT: u8 = 2;

    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        Button {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Output))),
            ],
            down: false,
        }
    }

    pub fn press(&mut self) {
        self.down = true;
    }

    pub fn release(&mut self) {
        self.down = false;
    }
}
impl Chip for Button {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        "virt_ic::Button"
    }
    fn get_pin_qty(&self) -> u8 {
        2
    }

    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str> {
        if pin > 0 && pin <= 2 {
            Ok(self.pin[pin as usize - 1].clone())
        } else {
            Err("Pin out of bounds")
        }
    }
    fn run(&mut self, _: std::time::Duration) {
        if self.down {
            self.pin[1].borrow_mut().state = self.pin[0].borrow().state.clone();
        } else {
            self.pin[1].borrow_mut().state = State::Undefined;
        }
    }

    fn save_data(&self) -> Vec<String> {
        vec![String::from(if self.down { "DOWN" } else { "UP" })]
    }
    fn load_data(&mut self, chip_data: &[String]) {
        self.down = chip_data[0] == "DOWN";
    }
}
