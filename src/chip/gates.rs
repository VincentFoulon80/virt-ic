use super::super::State;
use super::{Pin, PinType, Chip};
use std::cell::RefCell;
use std::rc::Rc;

/// # A chip with 4 bundled "OR" gates
/// 
/// # Diagram
/// ```
///        ---__---
///    A --|1   14|-- VCC
///    B --|2   13|-- E
///  A|B --|3   12|-- F
///    C --|4   11|-- E|F
///    D --|5   10|-- G
///  C|D --|6    9|-- H
///  GND --|7    8|-- G|H
///        --------
/// ```
#[derive(Debug)]
pub struct GateOr {
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for GateOr {
    fn default() -> Self {
        Self::new()
    }
}

impl GateOr {
    pub const A: u8 = 1;
    pub const B: u8 = 2;
    pub const A_OR_B: u8 = 3;
    pub const C: u8 = 4;
    pub const D: u8 = 5;
    pub const C_OR_D: u8 = 6;
    pub const E: u8 = 13;
    pub const F: u8 = 12;
    pub const E_OR_F: u8 = 11;
    pub const G: u8 = 10;
    pub const H: u8 = 9;
    pub const G_OR_H: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        GateOr {
            pin: [
                Rc::new(RefCell::new(Pin::new(1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(3, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(4, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(10, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(11, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(12, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(14, PinType::Input)))
            ]
        }
    }
}
impl Chip for GateOr {
    fn get_pin_qty(&self) -> u8 { 
        14
    }

    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str> { 
        if pin > 0 && pin <= 14 {
            Ok(self.pin[pin as usize-1].clone())
        } else {
            Err("Pin out of bounds")
        }
    }
    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {  
            // A && B    
            self.pin[2].borrow_mut().state = if self.pin[0].borrow().state == State::High || self.pin[1].borrow().state == State::High {State::High} else {State::Low};
            // C && D
            self.pin[5].borrow_mut().state = if self.pin[3].borrow().state == State::High || self.pin[4].borrow().state == State::High {State::High} else {State::Low};
            // E && F 
            self.pin[10].borrow_mut().state = if self.pin[11].borrow().state == State::High || self.pin[12].borrow().state == State::High {State::High} else {State::Low};
            // G && H 
            self.pin[7].borrow_mut().state = if self.pin[8].borrow().state == State::High || self.pin[9].borrow().state == State::High {State::High} else {State::Low};
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Low
            }
        }
    }
}


/// # A chip with 4 bundled "AND" gates
/// 
/// # Diagram
/// ```
///        ---__---
///    A --|1   14|-- VCC
///    B --|2   13|-- E
///  A&B --|3   12|-- F
///    C --|4   11|-- E&F
///    D --|5   10|-- G
///  C&D --|6    9|-- H
///  GND --|7    8|-- G&H
///        --------
/// ```
#[derive(Debug)]
pub struct GateAnd {
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for GateAnd {
    fn default() -> Self {
        Self::new()
    }
}

impl GateAnd {
    pub const A: u8 = 1;
    pub const B: u8 = 2;
    pub const A_AND_B: u8 = 3;
    pub const C: u8 = 4;
    pub const D: u8 = 5;
    pub const C_AND_D: u8 = 6;
    pub const E: u8 = 13;
    pub const F: u8 = 12;
    pub const E_AND_F: u8 = 11;
    pub const G: u8 = 10;
    pub const H: u8 = 9;
    pub const G_AND_H: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        GateAnd {
            pin: [
                Rc::new(RefCell::new(Pin::new(1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(3, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(4, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(10, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(11, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(12, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(14, PinType::Input)))
            ]
        }
    }
}
impl Chip for GateAnd {
    fn get_pin_qty(&self) -> u8 { 
        14
    }

    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str> { 
        if pin > 0 && pin <= 14 {
            Ok(self.pin[pin as usize-1].clone())
        } else {
            Err("Pin out of bounds")
        }
    }
    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {   
            // A && B  
            self.pin[2].borrow_mut().state = if self.pin[0].borrow().state == State::High && self.pin[1].borrow().state == State::High {State::High} else {State::Low};
            // C && D
            self.pin[5].borrow_mut().state = if self.pin[3].borrow().state == State::High && self.pin[4].borrow().state == State::High {State::High} else {State::Low};
            // E && F
            self.pin[10].borrow_mut().state = if self.pin[11].borrow().state == State::High && self.pin[12].borrow().state == State::High {State::High} else {State::Low};
            // G && H
            self.pin[7].borrow_mut().state = if self.pin[8].borrow().state == State::High && self.pin[9].borrow().state == State::High {State::High} else {State::Low};
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Undefined
            }
        }
    }
}

/// # A chip with 6 bundled "NOT" gates
/// 
/// # Diagram
/// ```
///        ---__---
///    A --|1   14|-- VCC
///   !A --|2   13|-- D
///    B --|3   12|-- !D
///   !B --|4   11|-- E
///    C --|5   10|-- !E
///   !C --|6    9|-- F
///  GND --|7    8|-- !F
///        --------
/// ```
#[derive(Debug)]
pub struct GateNot {
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for GateNot {
    fn default() -> Self {
        Self::new()
    }
}

impl GateNot {
    pub const A: u8 = 1;
    pub const NOT_A: u8 = 2;
    pub const B: u8 = 3;
    pub const NOT_B: u8 = 4;
    pub const C: u8 = 5;
    pub const NOT_C: u8 = 6;
    pub const D: u8 = 13;
    pub const NOT_D: u8 = 12;
    pub const E: u8 = 11;
    pub const NOT_E: u8 = 10;
    pub const F: u8 = 9;
    pub const NOT_F: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        GateNot {
            pin: [
                Rc::new(RefCell::new(Pin::new(1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(2, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(3, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(4, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(10, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(11, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(12, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(14, PinType::Input)))
            ]
        }
    }
}
impl Chip for GateNot {
    fn get_pin_qty(&self) -> u8 { 
        14
    }

    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str> { 
        if pin > 0 && pin <= 14 {
            Ok(self.pin[pin as usize-1].clone())
        } else {
            Err("Pin out of bounds")
        }
    }
    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {   
            // !A 
            self.pin[1].borrow_mut().state = if self.pin[0].borrow().state == State::High {State::Low} else {State::High};
            // !B
            self.pin[3].borrow_mut().state = if self.pin[2].borrow().state == State::High {State::Low} else {State::High};
            // !C 
            self.pin[5].borrow_mut().state = if self.pin[4].borrow().state == State::High {State::Low} else {State::High};
            // !D 
            self.pin[11].borrow_mut().state = if self.pin[12].borrow().state == State::High {State::Low} else {State::High};
            // !E 
            self.pin[9].borrow_mut().state = if self.pin[10].borrow().state == State::High {State::Low} else {State::High};
            // !F 
            self.pin[7].borrow_mut().state = if self.pin[8].borrow().state == State::High {State::Low} else {State::High};
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Undefined
            }
        }
    }
}