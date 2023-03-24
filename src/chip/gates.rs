pub mod and;
pub mod nand;
pub mod nor;
pub mod or;

use std::time::Duration;

pub use and::*;
pub use nand::*;
pub use nor::*;
pub use or::*;

use crate::{generate_chip, State};

use super::{ChipBuilder, ChipRunner, ChipSet, Pin, PinId, PinType};

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
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NotGate {
    pub vcc: Pin,
    pub gnd: Pin,
    pub a: Pin,
    pub not_a: Pin,
    pub b: Pin,
    pub not_b: Pin,
    pub c: Pin,
    pub not_c: Pin,
    pub d: Pin,
    pub not_d: Pin,
    pub e: Pin,
    pub not_e: Pin,
    pub f: Pin,
    pub not_f: Pin,
}
impl NotGate {
    pub const VCC: PinId = 14;
    pub const GND: PinId = 7;
    pub const A: PinId = 1;
    pub const NOT_A: PinId = 2;
    pub const B: PinId = 3;
    pub const NOT_B: PinId = 4;
    pub const C: PinId = 5;
    pub const NOT_C: PinId = 6;
    pub const D: PinId = 13;
    pub const NOT_D: PinId = 12;
    pub const E: PinId = 11;
    pub const NOT_E: PinId = 10;
    pub const F: PinId = 9;
    pub const NOT_F: PinId = 8;
}
impl ChipBuilder<ChipSet> for NotGate {
    fn build() -> ChipSet {
        ChipSet::NotGate(NotGate {
            vcc: Pin::from(PinType::Input),
            gnd: Pin::from(PinType::Output),
            a: Pin::from(PinType::Input),
            not_a: Pin::from(PinType::Output),
            b: Pin::from(PinType::Input),
            not_b: Pin::from(PinType::Output),
            c: Pin::from(PinType::Input),
            not_c: Pin::from(PinType::Output),
            d: Pin::from(PinType::Input),
            not_d: Pin::from(PinType::Output),
            e: Pin::from(PinType::Input),
            not_e: Pin::from(PinType::Output),
            f: Pin::from(PinType::Input),
            not_f: Pin::from(PinType::Output),
        })
    }
}

generate_chip!(
    NotGate,
    vcc: NotGate::VCC,
    gnd: NotGate::GND,
    a: NotGate::A,
    not_a: NotGate::NOT_A,
    b: NotGate::B,
    not_b: NotGate::NOT_B,
    c: NotGate::C,
    not_c: NotGate::NOT_C,
    d: NotGate::D,
    not_d: NotGate::NOT_D,
    e: NotGate::E,
    not_e: NotGate::NOT_E,
    f: NotGate::F,
    not_f: NotGate::NOT_F
);

impl ChipRunner for NotGate {
    fn run(&mut self, _: Duration) {
        if self.vcc.state.as_logic(3.3) == State::High {
            self.gnd.state = State::Low;
            self.not_a.state = State::from(!bool::from(self.a.state.as_logic(3.3)));
            self.not_b.state = State::from(!bool::from(self.b.state.as_logic(3.3)));
            self.not_c.state = State::from(!bool::from(self.c.state.as_logic(3.3)));
            self.not_d.state = State::from(!bool::from(self.d.state.as_logic(3.3)));
            self.not_e.state = State::from(!bool::from(self.e.state.as_logic(3.3)));
            self.not_f.state = State::from(!bool::from(self.f.state.as_logic(3.3)));
        }
    }
}
