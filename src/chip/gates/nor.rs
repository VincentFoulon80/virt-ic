use std::time::Duration;

use crate::{
    chip::{ChipBuilder, ChipRunner, ChipType, Pin, PinId, PinType},
    generate_chip, State,
};

/// # A chip with 4 bundled "NOR" gates
///
/// # Diagram
/// ```
///           ---__---
///       A --|1   14|-- VCC
///       B --|2   13|-- E
///  !(A&B) --|3   12|-- F
///       C --|4   11|-- !(E&F)
///       D --|5   10|-- G
///  !(C&D) --|6    9|-- H
///     GND --|7    8|-- !(G&H)
///           --------
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NorGate {
    pub vcc: Pin,
    pub gnd: Pin,
    pub a: Pin,
    pub b: Pin,
    pub ab: Pin,
    pub c: Pin,
    pub d: Pin,
    pub cd: Pin,
    pub e: Pin,
    pub f: Pin,
    pub ef: Pin,
    pub g: Pin,
    pub h: Pin,
    pub gh: Pin,
}

impl NorGate {
    pub const VCC: PinId = 14;
    pub const GND: PinId = 7;
    pub const A: PinId = 1;
    pub const B: PinId = 2;
    pub const AB: PinId = 3;
    pub const C: PinId = 4;
    pub const D: PinId = 5;
    pub const CD: PinId = 6;
    pub const E: PinId = 13;
    pub const F: PinId = 12;
    pub const EF: PinId = 11;
    pub const G: PinId = 10;
    pub const H: PinId = 9;
    pub const GH: PinId = 8;
}

impl ChipBuilder<ChipType> for NorGate {
    fn build() -> ChipType {
        ChipType::NorGate(NorGate {
            vcc: Pin::from(PinType::Input),
            gnd: Pin::from(PinType::Output),
            a: Pin::from(PinType::Input),
            b: Pin::from(PinType::Input),
            ab: Pin::from(PinType::Output),
            c: Pin::from(PinType::Input),
            d: Pin::from(PinType::Input),
            cd: Pin::from(PinType::Output),
            e: Pin::from(PinType::Input),
            f: Pin::from(PinType::Input),
            ef: Pin::from(PinType::Output),
            g: Pin::from(PinType::Input),
            h: Pin::from(PinType::Input),
            gh: Pin::from(PinType::Output),
        })
    }
}

generate_chip!(
    NorGate,
    vcc: NorGate::VCC,
    gnd: NorGate::GND,
    a: NorGate::A,
    b: NorGate::B,
    ab: NorGate::AB,
    c: NorGate::C,
    d: NorGate::D,
    cd: NorGate::CD,
    e: NorGate::E,
    f: NorGate::F,
    ef: NorGate::EF,
    g: NorGate::G,
    h: NorGate::H,
    gh: NorGate::GH
);

impl ChipRunner for NorGate {
    fn run(&mut self, _: Duration) {
        if self.vcc.state.as_logic(3.3) == State::High {
            self.gnd.state = State::Low;
            self.ab.state =
                State::from(self.a.state.as_logic(3.3).into() || self.b.state.as_logic(3.3).into());
            self.cd.state =
                State::from(self.c.state.as_logic(3.3).into() || self.d.state.as_logic(3.3).into());
            self.ef.state =
                State::from(self.e.state.as_logic(3.3).into() || self.f.state.as_logic(3.3).into());
            self.gh.state =
                State::from(self.g.state.as_logic(3.3).into() || self.h.state.as_logic(3.3).into());
        }
    }
}

/// # A chip with 3 bundled "3-Input NOR" gates
///
/// # Diagram
/// ```
///            ---__---
///        A --|1   14|-- VCC
///        B --|2   13|-- C
///        D --|3   12|-- !(A|B|C)
///        E --|4   11|-- G
///        F --|5   10|-- H
/// !(D|E|F) --|6    9|-- I
///      GND --|7    8|-- !(G|H|I)
///            --------
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThreeInputNorGate {
    pub vcc: Pin,
    pub gnd: Pin,
    pub a: Pin,
    pub b: Pin,
    pub c: Pin,
    pub abc: Pin,
    pub d: Pin,
    pub e: Pin,
    pub f: Pin,
    pub def: Pin,
    pub g: Pin,
    pub h: Pin,
    pub i: Pin,
    pub ghi: Pin,
}

impl ThreeInputNorGate {
    pub const VCC: PinId = 14;
    pub const GND: PinId = 7;
    pub const A: PinId = 1;
    pub const B: PinId = 2;
    pub const C: PinId = 13;
    pub const ABC: PinId = 12;
    pub const D: PinId = 3;
    pub const E: PinId = 4;
    pub const F: PinId = 5;
    pub const DEF: PinId = 6;
    pub const G: PinId = 11;
    pub const H: PinId = 10;
    pub const I: PinId = 9;
    pub const GHI: PinId = 8;
}

impl ChipBuilder<ChipType> for ThreeInputNorGate {
    fn build() -> ChipType {
        ChipType::ThreeInputNorGate(ThreeInputNorGate {
            vcc: Pin::from(PinType::Input),
            gnd: Pin::from(PinType::Output),
            a: Pin::from(PinType::Input),
            b: Pin::from(PinType::Input),
            c: Pin::from(PinType::Input),
            abc: Pin::from(PinType::Output),
            d: Pin::from(PinType::Input),
            e: Pin::from(PinType::Input),
            f: Pin::from(PinType::Input),
            def: Pin::from(PinType::Output),
            g: Pin::from(PinType::Input),
            h: Pin::from(PinType::Input),
            i: Pin::from(PinType::Input),
            ghi: Pin::from(PinType::Output),
        })
    }
}

generate_chip!(
    ThreeInputNorGate,
    vcc: ThreeInputNorGate::VCC,
    gnd: ThreeInputNorGate::GND,
    a: ThreeInputNorGate::A,
    b: ThreeInputNorGate::B,
    c: ThreeInputNorGate::C,
    abc: ThreeInputNorGate::ABC,
    d: ThreeInputNorGate::D,
    e: ThreeInputNorGate::E,
    f: ThreeInputNorGate::F,
    def: ThreeInputNorGate::DEF,
    g: ThreeInputNorGate::G,
    h: ThreeInputNorGate::H,
    i: ThreeInputNorGate::I,
    ghi: ThreeInputNorGate::GHI
);

impl ChipRunner for ThreeInputNorGate {
    fn run(&mut self, _: Duration) {
        if self.vcc.state.as_logic(3.3) == State::High {
            self.gnd.state = State::Low;
            self.abc.state = State::from(
                !(self.a.state.as_logic(3.3).into()
                    || self.b.state.as_logic(3.3).into()
                    || self.c.state.as_logic(3.3).into()),
            );
            self.def.state = State::from(
                !(self.d.state.as_logic(3.3).into()
                    || self.e.state.as_logic(3.3).into()
                    || self.f.state.as_logic(3.3).into()),
            );
            self.ghi.state = State::from(
                !(self.g.state.as_logic(3.3).into()
                    || self.h.state.as_logic(3.3).into()
                    || self.i.state.as_logic(3.3).into()),
            );
        }
    }
}
