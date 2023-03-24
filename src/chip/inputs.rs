use crate::{generate_chip, State};

use super::{ChipBuilder, ChipRunner, ChipSet, Pin, PinType};

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
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Button {
    down: bool,
    i: Pin,
    o: Pin,
}

impl Button {
    pub const I: usize = 1;
    pub const O: usize = 2;

    pub fn press(&mut self) {
        self.down = true;
    }

    pub fn release(&mut self) {
        self.down = false;
    }
}

generate_chip!(Button, i: Button::I, o: Button::O);

impl ChipBuilder<ChipSet> for Button {
    fn build() -> ChipSet {
        ChipSet::Button(Button {
            down: false,
            i: Pin::from(PinType::Input),
            o: Pin::from(PinType::Output),
        })
    }
}

impl ChipRunner for Button {
    fn run(&mut self, _: std::time::Duration) {
        if self.down {
            self.o.state = self.i.state;
        } else {
            self.o.state = State::Undefined
        }
    }
}
