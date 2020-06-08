use super::{Pin, PinType, State};
use std::cell::RefCell;
use std::rc::Rc;

/// A Trace that connects two chip's Pin
#[derive(Default, Debug)]
pub struct Trace {
    link: Vec<Rc<RefCell<Pin>>>
}

impl Trace {
    pub fn new() -> Trace {
        Trace {
            link: vec![]
        }
    }

    pub fn connect(&mut self, pin: Rc<RefCell<Pin>>) {
        self.link.push(pin)
    }

    pub fn communicate(&mut self) {
        let mut main_state = State::Undefined.to_current();
        for pin in self.link.iter() {
            if pin.borrow().pin_type == PinType::Output {
                let pin_state = pin.borrow().state.clone();
                if pin_state.to_current().voltage >  main_state.voltage {
                    main_state.voltage = pin_state.to_current().voltage;
                }
                if pin_state.to_current().amperage >  main_state.amperage {
                    main_state.amperage = pin_state.to_current().amperage;
                }
            }
        }
        let main_state = State::Analog(main_state);
        for pin in self.link.iter_mut() {
            if pin.borrow().pin_type != PinType::Output {
                pin.borrow_mut().state = main_state.clone();
            }
        }
    }
}