//! Chip trait, Pins and premade Chips
use super::State;
pub mod buttons;
pub mod clocks;
pub mod cpu;
pub mod gates;
pub mod generators;
pub mod memory;
use super::save::SavedChip;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

/// The type of a Pin, that can be Input or Output
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum PinType {
    Undefined,
    Input,
    Output,
    // Both // removed because it can cause issues on Trace::communicate(). It's better to swap the pin when needed
}

/// A chip's Pin. Can be of type Input or Output, and holds a State
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pin {
    pub parent: u128,
    pub number: u8,
    pub pin_type: PinType,
    pub state: State,
}
impl Pin {
    pub fn new(parent_uuid: u128, number: u8, pin_type: PinType) -> Pin {
        Pin {
            parent: parent_uuid,
            number,
            pin_type,
            state: State::Undefined,
        }
    }
}

pub struct ChipInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub data: String
}

/// Chip : a trait that represents chips on board
pub trait Chip {
    /// Give a unique id to maintain continuity when saving. This uuid must not maintain any information other that identity. When saving, this value will be used to link the traced pins to their respective chip.
    fn get_uuid(&self) -> u128;
    /// Give a unique name for the chip struct, it must be the same for every chips of the same struct. This value will be used to rebuild the correct Struct based on this name with the help of the chip factory
    fn get_type(&self) -> &str;
    /// Runs the chip for a certain amount of time
    fn run(&mut self, elapsed_time: std::time::Duration);
    /// Returns the number of pins the chip has
    fn get_pin_qty(&self) -> u8;
    /// Get a pin of the chip
    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str>;
    /// Get the state of the specified Pin
    fn get_pin_state(&mut self, pin: u8) -> State {
        if let Ok(pin) = self.get_pin(pin) {
            pin.borrow().state.clone()
        } else {
            State::Undefined
        }
    }
    /// Set the state of the specified Pin
    fn set_pin_state(&mut self, pin: u8, state: &State) {
        if let Ok(pin) = self.get_pin(pin) {
            pin.borrow_mut().state = state.clone();
        }
    }

    fn get_info(&self) -> ChipInfo;

    /// Save the chip to a SavedChip struct
    fn save(&self) -> SavedChip {
        SavedChip {
            uuid: self.get_uuid(),
            chip_type: String::from(self.get_type()),
            chip_data: self.save_data(),
        }
    }

    /// Create a Vec of String that must contain every information you need to restore your chip to a certain state. This will be saved in the resulting file.
    fn save_data(&self) -> Vec<String> {
        vec![]
    }

    /// Restore the chip from a SavedChip struct
    fn load(&mut self, saved_chip: &SavedChip) {
        self.load_data(&saved_chip.chip_data);
    }

    /// Using the array of String you provided in `save_data` , you must restore the state of your chip.
    fn load_data(&mut self, _chip_data: &[String]) {}
}
impl std::fmt::Debug for dyn Chip {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let info = self.get_info();
        fmt.write_str(info.name)?;
        fmt.write_str("\n")?;
        fmt.write_str(&info.data)?;
        Ok(())
    }
}

/// Factory function for chips that are built-in for the virt_ic crate.
///
/// You can provide your own factory function to the `Board::Load` to build your custom chips.
/// ```
/// pub fn my_custom_factory(chip_name: &str) -> Option<Box<dyn Chip>> {
///     if let Some(chip) = virt_ic::chip::virt_ic_chip_factory(chip_name) {
///         return chip;
///     } else {
///         // build your chips here
///         // with a match chip_name for example
///     }
/// }
/// ```
pub fn virt_ic_chip_factory(chip_name: &str) -> Option<Box<dyn Chip>> {
    match chip_name {
        "virt_ic::Button" => Some(Box::new(buttons::Button::new())),
        "virt_ic::Clock100Hz" => Some(Box::new(clocks::Clock100Hz::new())),
        "virt_ic::Clock1kHz" => Some(Box::new(clocks::Clock1kHz::new())),
        "virt_ic::SimpleCPU" => Some(Box::new(cpu::SimpleCPU::new())),
        "virt_ic::GateOr" => Some(Box::new(gates::GateOr::new())),
        "virt_ic::GateAnd" => Some(Box::new(gates::GateAnd::new())),
        "virt_ic::GateNot" => Some(Box::new(gates::GateNot::new())),
        "virt_ic::Generator" => Some(Box::new(generators::Generator::new())),
        "virt_ic::Ram256B" => Some(Box::new(memory::Ram256B::new())),
        "virt_ic::Rom256B" => Some(Box::new(memory::Rom256B::new())),
        _ => None,
    }
}
