use super::{Board, Chip, Pin, Socket};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedChip {
    pub uuid: u128,
    pub chip_type: String,
    pub chip_data: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedSocket {
    pub chip: Option<SavedChip>,
}
impl SavedSocket {
    pub fn new() -> Self {
        Self { chip: None }
    }
    pub fn set_chip(&mut self, chip: SavedChip) {
        self.chip = Some(chip);
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SavedTrace {
    pub pins: Vec<Pin>,
}
impl SavedTrace {
    pub fn new() -> Self {
        Self { pins: vec![] }
    }

    pub fn add_trace(&mut self, pin: Pin) {
        self.pins.push(pin);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedBoard {
    sockets: Vec<SavedSocket>,
    traces: Vec<SavedTrace>,
}

impl SavedBoard {
    pub fn new() -> Self {
        Self {
            sockets: vec![],
            traces: vec![],
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

        for s_socket in &self.sockets {
            let socket = board.new_socket();
            if let Some(s_chip) = &s_socket.chip {
                if let Some(chip) = chip_factory(&s_chip.chip_type) {
                    socket.borrow_mut().plug(chip);
                    socket.borrow_mut().load(s_chip);
                    loaded_chips.push((s_chip.uuid, socket.clone()));
                }
            }
        }

        for s_trace in &self.traces {
            let trace = board.new_trace();
            for s_pin in &s_trace.pins {
                for l_chip in &loaded_chips {
                    if s_pin.parent == l_chip.0 {
                        if let Ok(pin) = l_chip.1.borrow_mut().get_pin(s_pin.number) {
                            pin.borrow_mut().state = s_pin.state.clone();
                            pin.borrow_mut().pin_type = s_pin.pin_type.clone();
                            trace.borrow_mut().connect(pin.clone());
                        }
                    }
                }
            }
        }
        board
    }
}
