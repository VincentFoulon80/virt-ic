pub mod clocks;
pub mod cpu;
pub mod gates;
pub mod generators;
pub mod inputs;
pub mod memories;
pub mod outputs;

use std::{fmt::Debug, time::Duration};

use crate::State;

pub type PinId = usize;

pub trait ChipBuilder<C: Chip> {
    fn build() -> C;
}

pub trait ChipRunner {
    fn run(&mut self, tick_duration: Duration);
}

pub trait Chip: Debug + Clone + ChipRunner {
    fn list_pins(&self) -> Vec<(PinId, &Pin)>;
    fn get_pin(&self, pin: PinId) -> Option<&Pin>;
    fn get_pin_mut(&mut self, pin: PinId) -> Option<&mut Pin>;
}

#[macro_export]
macro_rules! impl_chip_type {
    ( $type:ident: ($($variant:ident),*)) => {
        impl $crate::chip::Chip for $type {
            fn list_pins(&self) -> ::std::vec::Vec<($crate::chip::PinId, &$crate::chip::Pin)> {
                match self {
                    $($type::$variant(chip) => chip.list_pins()),*
                }
            }

            fn get_pin(&self, pin: $crate::chip::PinId) -> ::std::option::Option<&$crate::chip::Pin> {
                match self {
                    $($type::$variant(chip) => chip.get_pin(pin)),*
                }
            }

            fn get_pin_mut(&mut self, pin: $crate::chip::PinId) -> ::std::option::Option<&mut $crate::chip::Pin> {
                match self {
                    $($type::$variant(chip) => chip.get_pin_mut(pin)),*
                }
            }
        }
        impl $crate::chip::ChipRunner for $type {
            fn run(&mut self, tick_duration: ::std::time::Duration) {
                match self {
                    $($type::$variant(chip) => chip.run(tick_duration)),*
                }
            }
        }
    };
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChipSet {
    AndGate(gates::AndGate),
    ThreeInputAndGate(gates::ThreeInputAndGate),
    NandGate(gates::NandGate),
    ThreeInputNandGate(gates::ThreeInputNandGate),
    OrGate(gates::OrGate),
    ThreeInputOrGate(gates::ThreeInputOrGate),
    ThreeInputNorGate(gates::ThreeInputNorGate),
    NorGate(gates::NorGate),
    NotGate(gates::NotGate),
    Generator(generators::Generator),
    Clock(clocks::Clock),
    Ram256B(memories::Ram256B),
    Ram8KB(memories::Ram8KB),
    Rom256B(memories::Rom256B),
    Rom8KB(memories::Rom8KB),
    Button(inputs::Button),
    Nes6502(Box<cpu::nes6502::Nes6502>),
    SevenSegmentDecoder(outputs::SevenSegmentsDecoder),
    SegmentDisplay(outputs::SegmentDisplay),
}

#[deprecated(since = "0.5.1", note = "Please use `ChipSet` instead")]
pub type ChipType = ChipSet;

impl_chip_type!(
    ChipSet:
        (
            AndGate,
            ThreeInputAndGate,
            NandGate,
            ThreeInputNandGate,
            OrGate,
            ThreeInputOrGate,
            NorGate,
            ThreeInputNorGate,
            NotGate,
            Generator,
            Clock,
            Ram256B,
            Ram8KB,
            Rom256B,
            Rom8KB,
            Button,
            Nes6502,
            SevenSegmentDecoder,
            SegmentDisplay
        )
);

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PinType {
    #[default]
    Floating,
    Input,
    Output,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pin {
    pub pin_type: PinType,
    pub state: State,
}

impl Pin {
    /// Read a given set of pins
    pub fn read(pins: &[&Pin]) -> usize {
        let mut sum = 0;
        for (i, pin) in pins.iter().enumerate() {
            if pin.state.into() {
                sum += 1 << i;
            }
        }
        sum
    }

    /// Read a given set of pins
    pub fn read_threshold(pins: &[&Pin], input_threshold: f32) -> usize {
        let mut sum = 0;
        for (i, pin) in pins.iter().enumerate() {
            if pin.state.as_logic(input_threshold).into() {
                sum += 1 << i;
            }
        }
        sum
    }

    /// Write a given value to a set of pins.
    /// If the value overflows, return true
    pub fn write(pins: &mut [&mut Pin], mut value: usize) -> bool {
        for (i, pin) in pins.iter_mut().enumerate() {
            pin.state = State::from((value & 1 << i) != 0);
            value &= usize::MAX - (1 << i);
        }
        value > 0
    }
}

impl From<PinType> for Pin {
    fn from(value: PinType) -> Self {
        Pin {
            pin_type: value,
            state: State::default(),
        }
    }
}

#[macro_export]
macro_rules! generate_chip {
    ($struct_name:ident, $($pin_name:ident: $pin_id:expr),*) => {
        impl $crate::chip::Chip for $struct_name {
            fn list_pins(&self) -> ::std::vec::Vec<($crate::chip::PinId, &$crate::chip::Pin)> {
                vec![
                    $( ($pin_id, &self.$pin_name), )*
                ]
            }

            fn get_pin(&self, pin: $crate::chip::PinId) -> ::std::option::Option<&$crate::chip::Pin> {
                match pin {
                    $( pin_id if pin_id == $pin_id => ::std::option::Option::Some(&self.$pin_name), )*
                    _ => ::std::option::Option::None,
                }
            }

            fn get_pin_mut(&mut self, pin: $crate::chip::PinId) -> ::std::option::Option<&mut $crate::chip::Pin> {
                match pin {
                    $( pin_id if pin_id == $pin_id => ::std::option::Option::Some(&mut self.$pin_name), )*
                    _ => ::std::option::Option::None,
                }
            }
        }
    };
}
