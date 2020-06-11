//! Clocks that pulse at different speeds
use crate::save::SavedChip;
use crate::State;
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
    uuid: u128,
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
        let uuid = uuid::Uuid::new_v4().as_u128();
        Clock100Hz {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 3, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 4, PinType::Input))),
            ],
            timer: Duration::new(0,0),
            active: false,
        }
    }
}
impl Chip for Clock100Hz {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        "virt_ic::Clock100Hz"
    }
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
        const LIMIT: Duration = Duration::from_millis(10);
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

    fn save(&self) -> SavedChip {
        SavedChip {
            uuid: self.uuid,
            chip_type: String::from(self.get_type()),
            chip_data: vec![
                String::from(if self.active {"ON"} else {"OFF"}),
                ron::to_string(&self.timer).unwrap()
            ]
        }
    }
    fn load(&mut self, s_chip: &SavedChip) {
        let timer: Duration = ron::from_str(&s_chip.chip_data[1]).unwrap();
        self.active = s_chip.chip_data[0] == "ON";
        self.timer = timer;
    }
}

/// A 1 kHz simple clock
/// CLK: clock
/// ```
///        --------
///  CLK --|1    4|-- VCC
///  GND --|2    3|-- UNUSED
///        --------
/// ```
#[derive(Debug)]
pub struct Clock1kHz {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 4],
    timer: Duration,
    active: bool
}
impl Default for Clock1kHz {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock1kHz {
    pub const CLK: u8 = 1;
    pub const VCC: u8 = 4;
    pub const GND: u8 = 2;

    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        Clock1kHz {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 3, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 4, PinType::Input))),
            ],
            timer: Duration::new(0,0),
            active: false,
        }
    }
}
impl Chip for Clock1kHz {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        "virt_ic::Clock1kHz"
    }
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
        const LIMIT: Duration = Duration::from_millis(1);
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
    
    fn save(&self) -> SavedChip {
        SavedChip {
            uuid: self.uuid,
            chip_type: String::from(self.get_type()),
            chip_data: vec![
                String::from(if self.active {"ON"} else {"OFF"}),
                ron::to_string(&self.timer).unwrap()
            ]
        }
    }
    fn load(&mut self, s_chip: &SavedChip) {
        let timer: Duration = ron::from_str(&s_chip.chip_data[1]).unwrap();
        self.active = s_chip.chip_data[0] == "ON";
        self.timer = timer;
    }
}