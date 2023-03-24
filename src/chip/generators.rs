use std::time::Duration;

use crate::State;

use super::{Chip, ChipBuilder, ChipRunner, ChipType, Pin, PinId, PinType};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Generator {
    state: State,
    pub pin: Pin,
}

impl Generator {
    pub const OUT: PinId = 1;

    pub fn with_state(mut self, state: State) -> Self {
        self.state = state;
        self.pin.state = state;
        self
    }
}

impl ChipBuilder<Generator> for Generator {
    fn build() -> Generator {
        Generator {
            state: State::High,
            pin: Pin {
                pin_type: PinType::Output,
                state: State::High,
            },
        }
    }
}

impl From<Generator> for ChipType {
    fn from(value: Generator) -> Self {
        ChipType::Generator(value)
    }
}

impl Chip for Generator {
    fn list_pins(&self) -> Vec<(super::PinId, &Pin)> {
        vec![(Generator::OUT, &self.pin)]
    }

    fn get_pin(&self, _pin: super::PinId) -> Option<&Pin> {
        Some(&self.pin)
    }

    fn get_pin_mut(&mut self, _pin: super::PinId) -> Option<&mut Pin> {
        Some(&mut self.pin)
    }
}

impl ChipRunner for Generator {
    fn run(&mut self, _: Duration) {
        self.pin.state = self.state
    }
}
