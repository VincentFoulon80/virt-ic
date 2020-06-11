use serde::{Serialize, Deserialize};
use super::{Pin, Board, Chip, Socket};
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, Serialize, Deserialize)]
pub struct SavedChip {
    pub uuid: u128,
    pub chip_type: String,
    pub chip_data: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedSocket {
    pub chip: Option<SavedChip>
}
impl SavedSocket {
    pub fn new() -> SavedSocket {
        SavedSocket {
            chip: None
        }
    }
    pub fn set_chip(&mut self, chip: SavedChip) {
        self.chip = Some(chip)
    }
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SavedTrace {
    pub pins: Vec<Pin>
}
impl SavedTrace {
    pub fn new() -> SavedTrace {
        SavedTrace {
            pins: vec![]
        }
    }

    pub fn add_trace(&mut self, pin: Pin) {
        self.pins.push(pin);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedBoard {
    sockets: Vec<SavedSocket>,
    traces: Vec<SavedTrace>
}

impl SavedBoard {
    pub fn new() -> Self {
        SavedBoard{
            sockets: vec![],
            traces: vec![]
        }
    }
    pub fn add_trace(&mut self, trace: SavedTrace) {
        self.traces.push(trace);
    }
    pub fn add_socket(&mut self, socket: SavedSocket) {
        self.sockets.push(socket);
    }

    pub fn build_board(&self, chip_factory: &dyn Fn(&str) -> Option<Box<dyn Chip>>) -> Board {
        let mut board = Board::new();
        let mut loaded_chips: Vec<(u128, Rc<RefCell<Socket>>)> = vec![];

        for s_socket in self.sockets.iter() {
            let socket = board.new_socket();
            if let Some(s_chip) = &s_socket.chip {
                if let Some(chip) = chip_factory(&s_chip.chip_type) {
                    socket.borrow_mut().plug(chip);
                    socket.borrow_mut().load(s_chip);
                    loaded_chips.push((s_chip.uuid, socket.clone()));
                }
            }
        }

        for s_trace in self.traces.iter() {
            let trace = board.new_trace();
            for s_pin in s_trace.pins.iter() {
                for l_chip in loaded_chips.iter() {
                    if s_pin.parent == l_chip.0 {
                        l_chip.1.borrow_mut().set_pin_state(s_pin.number, &s_pin.state);
                        l_chip.1.borrow_mut().get_pin(s_pin.number).unwrap().borrow_mut().pin_type = s_pin.pin_type.clone();
                        trace.borrow_mut().connect(l_chip.1.borrow_mut().get_pin(s_pin.number).unwrap().clone())
                    }
                }
            }
        }
        board
    }
}