use super::{Chip, Pin, PinType, State};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Socket {
    chip: Option<Box<dyn Chip>>,
}

impl Socket {
    pub fn new() -> Socket {
        Socket {
            chip: None
        }
    }

    pub fn with(chip: Box<dyn Chip>) -> Socket {
        Socket {
            chip: Some(chip)
        }
    }

    pub fn plug(&mut self, chip: Box<dyn Chip>) {
        self.chip = Some(chip);
    }

    pub fn has_chip(&self) -> bool {
        self.chip.is_some()
    }

    pub fn get_chip(&mut self) -> &mut Option<Box<dyn Chip>> {
        &mut self.chip
    }

    pub fn get_pin_type(&mut self, pin: u8) -> PinType {
        if self.chip.is_some() {
            self.chip.as_mut().unwrap().get_pin(pin).unwrap().borrow().pin_type.clone()
        } else {
            PinType::Undefined
        }
    }

    pub fn set_pin_type(&mut self, pin: u8, pin_type: &PinType) {
        if self.chip.is_some() {
            self.chip.as_mut().unwrap().get_pin(pin).unwrap().borrow_mut().pin_type = pin_type.clone();
        }
    }
}

impl Chip for Socket {
    fn get_pin_qty(&self) -> u8 {
        if self.chip.is_some() {
            self.chip.as_ref().unwrap().get_pin_qty()
        } else {
            0
        }
    }

    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str> {
        if self.chip.is_some() {
            self.chip.as_mut().unwrap().get_pin(pin)
        } else {
            Err("No chip connected")
        }
    }

    fn get_pin_state(&mut self, pin: u8) -> State {
        if self.chip.is_some() {
            self.chip.as_mut().unwrap().get_pin(pin).unwrap().borrow().state.clone()
        } else {
            State::Undefined
        }
    }

    fn set_pin_state(&mut self, pin: u8, state: &State) {
        if self.chip.is_some() {
            self.chip.as_mut().unwrap().get_pin(pin).unwrap().borrow_mut().state = state.clone();
        }
    }

    fn run(&mut self, elapsed_time: std::time::Duration) {
        if self.chip.is_some() {
            self.chip.as_mut().unwrap().run(elapsed_time)
        }
    }
}