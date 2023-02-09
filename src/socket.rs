use super::{Chip, ChipInfo, Pin, PinType, State};
use std::cell::RefCell;
use std::rc::Rc;

/// A Socket that holds a Chip
#[derive(Default, Debug)]
pub struct Socket {
    chip: Option<Box<dyn Chip>>,
}

impl Socket {
    #[must_use]
    pub fn new() -> Self {
        Self { chip: None }
    }

    pub fn with(chip: Box<dyn Chip>) -> Self {
        Self { chip: Some(chip) }
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
        self.chip.as_mut().map_or(PinType::Undefined, |chip| {
            chip.get_pin(pin)
                .map_or(PinType::Undefined, |pin| pin.borrow().pin_type.clone())
        })
    }

    pub fn set_pin_type(&mut self, pin: u8, pin_type: &PinType) {
        if let Some(chip) = self.chip.as_mut() {
            if let Ok(pin) = chip.get_pin(pin) {
                pin.borrow_mut().pin_type = pin_type.clone();
            }
        }
    }
}

impl Chip for Socket {
    fn get_uuid(&self) -> u128 {
        self.chip.as_ref().map_or(0, |chip| chip.get_uuid())
    }
    fn get_type(&self) -> &str {
        self.chip.as_ref().map_or("NULL", |chip| chip.get_type())
    }

    fn get_pin_qty(&self) -> u8 {
        self.chip.as_ref().map_or(0, |chip| chip.get_pin_qty())
    }

    fn _get_pin(&mut self, _: u8) -> Rc<RefCell<Pin>> {
        panic!("_get_pin is not intended to be called for a Socket !");
    }

    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str> {
        self.chip
            .as_mut()
            .map_or(Err("No chip connected"), |chip| chip.get_pin(pin))
    }

    fn get_pin_state(&mut self, pin: u8) -> State {
        self.chip.as_mut().map_or(State::Undefined, |chip| {
            chip.get_pin(pin)
                .map_or(State::Undefined, |pin| pin.borrow().state.clone())
        })
    }

    fn set_pin_state(&mut self, pin: u8, state: &State) {
        if let Some(chip) = self.chip.as_mut() {
            if let Ok(pin) = chip.get_pin(pin) {
                pin.borrow_mut().state = state.clone();
            }
        }
    }

    fn get_info(&self) -> ChipInfo {
        self.chip.as_ref().map_or_else(
            || ChipInfo {
                name: "Empty Socket",
                description: "Socket without any chip plugged in it",
                data: String::new(),
            },
            |chip| chip.get_info(),
        )
    }

    fn run(&mut self, elapsed_time: std::time::Duration) {
        if let Some(chip) = self.chip.as_mut() {
            chip.run(elapsed_time);
        }
    }
    fn save_data(&self) -> Vec<String> {
        self.chip
            .as_ref()
            .map_or_else(std::vec::Vec::new, |chip| chip.save_data())
    }
    fn load_data(&mut self, s_chip: &[String]) {
        if let Some(chip) = self.chip.as_mut() {
            chip.load_data(s_chip);
        }
    }
}
