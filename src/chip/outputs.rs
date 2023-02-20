pub mod helpers;

pub use helpers::*;

use crate::{generate_chip, State};

use super::{ChipBuilder, ChipRunner, ChipType, Pin, PinType};

/// Simple 7-Segment display
///
/// # Diagram
/// ```txt
///     ------------
///  a -|1        9|- VCC
///  b -|2   ──    |
///  c -|3  |  |   |
///  d -|4   ──    |
///  e -|5  |  |   |
///  f -|6   ──    |
///  g -|7        8|- GND
///     ------------
/// ```
///
/// pins-to-segment map:
/// ```txt
///    a
///   ───
/// f|   |b
///   ─g─
/// e|   |c
///   ───
///    d
/// ```
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SegmentDisplay {
    pub vcc: Pin,
    pub gnd: Pin,
    pub a: Pin,
    pub b: Pin,
    pub c: Pin,
    pub d: Pin,
    pub e: Pin,
    pub f: Pin,
    pub g: Pin,
}

impl SegmentDisplay {
    pub const VCC: usize = 9;
    pub const GND: usize = 8;
    pub const A: usize = 1;
    pub const B: usize = 2;
    pub const C: usize = 3;
    pub const D: usize = 4;
    pub const E: usize = 5;
    pub const F: usize = 6;
    pub const G: usize = 7;
}

generate_chip!(
    SegmentDisplay,
    vcc: SegmentDisplay::VCC,
    gnd: SegmentDisplay::GND,
    a: SegmentDisplay::A,
    b: SegmentDisplay::B,
    c: SegmentDisplay::C,
    d: SegmentDisplay::D,
    e: SegmentDisplay::E,
    f: SegmentDisplay::F,
    g: SegmentDisplay::G
);

impl ChipBuilder<ChipType> for SegmentDisplay {
    fn build() -> ChipType {
        ChipType::SegmentDisplay(SegmentDisplay {
            vcc: Pin::from(PinType::Input),
            gnd: Pin::from(PinType::Output),
            a: Pin::from(PinType::Input),
            b: Pin::from(PinType::Input),
            c: Pin::from(PinType::Input),
            d: Pin::from(PinType::Input),
            e: Pin::from(PinType::Input),
            f: Pin::from(PinType::Input),
            g: Pin::from(PinType::Input),
        })
    }
}

impl ChipRunner for SegmentDisplay {
    fn run(&mut self, _: std::time::Duration) {
        if self.vcc.state.into() {
            self.gnd.state = State::Low;
        }
    }
}

impl ToString for SegmentDisplay {
    fn to_string(&self) -> String {
        if self.vcc.state.into() {
            format!(
                " {} \n{}  {}\n {} \n{}  {}\n {} ",
                if self.a.state.into() { "──" } else { "  " },
                if self.f.state.into() { "|" } else { " " },
                if self.b.state.into() { "|" } else { " " },
                if self.g.state.into() { "──" } else { "  " },
                if self.e.state.into() { "|" } else { " " },
                if self.c.state.into() { "|" } else { " " },
                if self.d.state.into() { "──" } else { "  " }
            )
        } else {
            String::from("    \n    \n    \n    \n    ")
        }
    }
}

impl SegmentDisplay {
    pub fn as_char(&self) -> char {
        if self.vcc.state.into() {
            let segments = Pin::read(&[
                &self.g, &self.f, &self.e, &self.d, &self.c, &self.b, &self.a,
            ]);
            match segments {
                0b0000000 => ' ',
                0b1111110 => '0',
                0b0110000 => '1',
                0b1101101 => '2',
                0b1111001 => '3',
                0b0110011 => '4',
                0b1011011 => '5',
                0b1011111 => '6',
                0b1110000 | 0b1110010 => '7',
                0b1111111 => '8',
                0b1111011 => '9',
                0b1110111 => 'A',
                0b0011111 => 'b',
                0b1001110 => 'C',
                0b0001101 => 'c',
                0b0111101 => 'd',
                0b1001111 => 'E',
                0b1000111 => 'F',
                0b1011110 => 'G',
                0b0110111 => 'H',
                0b0010111 => 'h',
                0b0111100 => 'J',
                0b0001110 => 'L',
                0b0001100 => 'l',
                0b1110110 => 'M',
                0b0010101 => 'n',
                0b0011101 => 'o',
                0b1100111 => 'p',
                0b1110011 => 'q',
                0b0001111 => 't',
                0b0111110 => 'U',
                0b0011100 => 'u',
                0b0111011 => 'y',
                0b0001000 => '_',
                0b0000001 => '-',
                0b0001001 | 0b1001000 => '=',
                _ => '?',
            }
        } else {
            ' '
        }
    }
}
