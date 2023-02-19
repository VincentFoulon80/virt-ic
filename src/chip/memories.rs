use std::time::Duration;

use rand::random;

use crate::{generate_chip, State};

use super::{ChipBuilder, ChipRunner, ChipType, Pin, PinType};

/// # A 256-bytes RAM chip
///
/// # Diagram
/// CS: Chip Select (active low)
/// WE: Write Enable (active low)
/// OE: Output Enable (active low)
/// A0-7: Addresses
/// IO0-7: Input/Output
/// ```
///        ---__---
///  !CS --|1   22|-- VCC
///  !WE --|2   21|-- UNUSED
///  !OE --|3   20|-- IO7
///   A0 --|4   19|-- IO6
///   A1 --|5   18|-- IO5
///   A2 --|6   17|-- IO4
///   A3 --|7   16|-- IO3
///   A4 --|8   15|-- IO2
///   A5 --|9   14|-- IO1
///   A6 --|10  13|-- IO0
///  GND --|11  12|-- A7
///        --------
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ram256B {
    powered: bool,
    ram: Vec<u8>,
    pub vcc: Pin,
    pub gnd: Pin,
    pub cs: Pin,
    pub we: Pin,
    pub oe: Pin,
    pub a0: Pin,
    pub a1: Pin,
    pub a2: Pin,
    pub a3: Pin,
    pub a4: Pin,
    pub a5: Pin,
    pub a6: Pin,
    pub a7: Pin,
    pub io0: Pin,
    pub io1: Pin,
    pub io2: Pin,
    pub io3: Pin,
    pub io4: Pin,
    pub io5: Pin,
    pub io6: Pin,
    pub io7: Pin,
}

impl Ram256B {
    pub const CS: usize = 1;
    pub const WE: usize = 2;
    pub const OE: usize = 3;
    pub const A0: usize = 4;
    pub const A1: usize = 5;
    pub const A2: usize = 6;
    pub const A3: usize = 7;
    pub const A4: usize = 8;
    pub const A5: usize = 9;
    pub const A6: usize = 10;
    pub const A7: usize = 12;
    pub const IO0: usize = 13;
    pub const IO1: usize = 14;
    pub const IO2: usize = 15;
    pub const IO3: usize = 16;
    pub const IO4: usize = 17;
    pub const IO5: usize = 18;
    pub const IO6: usize = 19;
    pub const IO7: usize = 20;
    pub const VCC: usize = 22;
    pub const GND: usize = 11;

    fn set_io_type(&mut self, pin_type: PinType) {
        self.io0.pin_type = pin_type;
        self.io1.pin_type = pin_type;
        self.io2.pin_type = pin_type;
        self.io3.pin_type = pin_type;
        self.io4.pin_type = pin_type;
        self.io5.pin_type = pin_type;
        self.io6.pin_type = pin_type;
        self.io7.pin_type = pin_type;
    }
}

generate_chip!(
    Ram256B,
    cs: Ram256B::CS,
    we: Ram256B::WE,
    oe: Ram256B::OE,
    a0: Ram256B::A0,
    a1: Ram256B::A1,
    a2: Ram256B::A2,
    a3: Ram256B::A3,
    a4: Ram256B::A4,
    a5: Ram256B::A5,
    a6: Ram256B::A6,
    a7: Ram256B::A7,
    io0: Ram256B::IO0,
    io1: Ram256B::IO1,
    io2: Ram256B::IO2,
    io3: Ram256B::IO3,
    io4: Ram256B::IO4,
    io5: Ram256B::IO5,
    io6: Ram256B::IO6,
    io7: Ram256B::IO7,
    vcc: Ram256B::VCC,
    gnd: Ram256B::GND
);

impl ChipBuilder<ChipType> for Ram256B {
    fn build() -> ChipType {
        ChipType::Ram256B(Ram256B {
            powered: false,
            ram: Vec::from([0; 256]),
            vcc: Pin::from(PinType::Input),
            gnd: Pin::from(PinType::Output),
            cs: Pin::from(PinType::Input),
            we: Pin::from(PinType::Input),
            oe: Pin::from(PinType::Input),
            a0: Pin::from(PinType::Input),
            a1: Pin::from(PinType::Input),
            a2: Pin::from(PinType::Input),
            a3: Pin::from(PinType::Input),
            a4: Pin::from(PinType::Input),
            a5: Pin::from(PinType::Input),
            a6: Pin::from(PinType::Input),
            a7: Pin::from(PinType::Input),
            io0: Pin::from(PinType::Floating),
            io1: Pin::from(PinType::Floating),
            io2: Pin::from(PinType::Floating),
            io3: Pin::from(PinType::Floating),
            io4: Pin::from(PinType::Floating),
            io5: Pin::from(PinType::Floating),
            io6: Pin::from(PinType::Floating),
            io7: Pin::from(PinType::Floating),
        })
    }
}

impl ChipRunner for Ram256B {
    fn run(&mut self, _: Duration) {
        if self.vcc.state.as_logic(1.0) == State::High {
            if !self.powered {
                for i in 0..256 {
                    self.ram[i] = random::<u8>();
                }
                self.powered = true;
            }
            self.gnd.state = State::Low;

            // check Chip Select (active low)
            if self.cs.state == State::Low {
                // check Write Enable (active low)
                if self.we.state == State::Low {
                    // IO = Input
                    self.set_io_type(PinType::Input);
                    // read data on IO pins
                    let addr = Pin::read(&[
                        &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                        &self.a7,
                    ]);
                    self.ram[addr] = Pin::read(&[
                        &self.io0, &self.io1, &self.io2, &self.io3, &self.io4, &self.io5,
                        &self.io6, &self.io7,
                    ]) as u8;
                } else if self.oe.state == State::Low {
                    // IO = Output
                    self.set_io_type(PinType::Output);

                    // display data on IO pins
                    let addr = Pin::read(&[
                        &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                        &self.a7,
                    ]);
                    Pin::write(
                        &mut [
                            &mut self.io0,
                            &mut self.io1,
                            &mut self.io2,
                            &mut self.io3,
                            &mut self.io4,
                            &mut self.io5,
                            &mut self.io6,
                            &mut self.io7,
                        ],
                        self.ram[addr] as usize,
                    );
                } else {
                    self.set_io_type(PinType::Floating);
                }
            } else {
                self.set_io_type(PinType::Floating);
            }
        } else if self.powered {
            self.set_io_type(PinType::Floating);
            self.powered = false;
        }
    }
}

impl ToString for Ram256B {
    fn to_string(&self) -> std::string::String {
        let mut string = String::from(
            "ADR| 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
---+------------------------------------------------",
        );
        for (addr, byte) in self.ram.iter().enumerate() {
            if addr % 16 == 0 {
                string.push_str(&format!("\n {addr:02X}|"));
            }
            string.push_str(&format!(
                "{}{byte:02X}",
                if self.cs.state.as_logic(3.3) == State::Low
                    && Pin::read(&[
                        &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                        &self.a7
                    ]) == addr
                {
                    ">"
                } else {
                    " "
                }
            ));
        }
        string.push('\n');
        string
    }
}

/// # A 256-bytes ROM chip
///
/// # Diagram
/// CS: Chip Select (active low)
/// OE: Output Enable (active low)
/// A0-7: Addresses
/// IO0-7: Input/Output
/// ```
///         ---__---
///   !CS --|1   22|-- VCC
/// UNUSED--|2   21|-- UNUSED
///   !OE --|3   20|-- IO7
///    A0 --|4   19|-- IO6
///    A1 --|5   18|-- IO5
///    A2 --|6   17|-- IO4
///    A3 --|7   16|-- IO3
///    A4 --|8   15|-- IO2
///    A5 --|9   14|-- IO1
///    A6 --|10  13|-- IO0
///   GND --|11  12|-- A7
///         --------
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rom256B {
    powered: bool,
    rom: Vec<u8>,
    pub vcc: Pin,
    pub gnd: Pin,
    pub cs: Pin,
    pub oe: Pin,
    pub a0: Pin,
    pub a1: Pin,
    pub a2: Pin,
    pub a3: Pin,
    pub a4: Pin,
    pub a5: Pin,
    pub a6: Pin,
    pub a7: Pin,
    pub io0: Pin,
    pub io1: Pin,
    pub io2: Pin,
    pub io3: Pin,
    pub io4: Pin,
    pub io5: Pin,
    pub io6: Pin,
    pub io7: Pin,
}

impl Rom256B {
    pub const CS: usize = 1;
    pub const OE: usize = 3;
    pub const A0: usize = 4;
    pub const A1: usize = 5;
    pub const A2: usize = 6;
    pub const A3: usize = 7;
    pub const A4: usize = 8;
    pub const A5: usize = 9;
    pub const A6: usize = 10;
    pub const A7: usize = 12;
    pub const IO0: usize = 13;
    pub const IO1: usize = 14;
    pub const IO2: usize = 15;
    pub const IO3: usize = 16;
    pub const IO4: usize = 17;
    pub const IO5: usize = 18;
    pub const IO6: usize = 19;
    pub const IO7: usize = 20;
    pub const VCC: usize = 22;
    pub const GND: usize = 11;

    fn set_io_type(&mut self, pin_type: PinType) {
        self.io0.pin_type = pin_type;
        self.io1.pin_type = pin_type;
        self.io2.pin_type = pin_type;
        self.io3.pin_type = pin_type;
        self.io4.pin_type = pin_type;
        self.io5.pin_type = pin_type;
        self.io6.pin_type = pin_type;
        self.io7.pin_type = pin_type;
    }

    pub fn set_data(mut self, data: [u8; 256]) -> Self {
        self.rom = Vec::from(data);
        self
    }
}

generate_chip!(
    Rom256B,
    cs: Rom256B::CS,
    oe: Rom256B::OE,
    a0: Rom256B::A0,
    a1: Rom256B::A1,
    a2: Rom256B::A2,
    a3: Rom256B::A3,
    a4: Rom256B::A4,
    a5: Rom256B::A5,
    a6: Rom256B::A6,
    a7: Rom256B::A7,
    io0: Rom256B::IO0,
    io1: Rom256B::IO1,
    io2: Rom256B::IO2,
    io3: Rom256B::IO3,
    io4: Rom256B::IO4,
    io5: Rom256B::IO5,
    io6: Rom256B::IO6,
    io7: Rom256B::IO7,
    vcc: Rom256B::VCC,
    gnd: Rom256B::GND
);

impl ChipBuilder<Rom256B> for Rom256B {
    fn build() -> Rom256B {
        Rom256B {
            powered: false,
            rom: Vec::from([0; 256]),
            vcc: Pin::from(PinType::Input),
            gnd: Pin::from(PinType::Output),
            cs: Pin::from(PinType::Input),
            oe: Pin::from(PinType::Input),
            a0: Pin::from(PinType::Input),
            a1: Pin::from(PinType::Input),
            a2: Pin::from(PinType::Input),
            a3: Pin::from(PinType::Input),
            a4: Pin::from(PinType::Input),
            a5: Pin::from(PinType::Input),
            a6: Pin::from(PinType::Input),
            a7: Pin::from(PinType::Input),
            io0: Pin::from(PinType::Floating),
            io1: Pin::from(PinType::Floating),
            io2: Pin::from(PinType::Floating),
            io3: Pin::from(PinType::Floating),
            io4: Pin::from(PinType::Floating),
            io5: Pin::from(PinType::Floating),
            io6: Pin::from(PinType::Floating),
            io7: Pin::from(PinType::Floating),
        }
    }
}

impl From<Rom256B> for ChipType {
    fn from(value: Rom256B) -> Self {
        ChipType::Rom256B(value)
    }
}

impl ChipRunner for Rom256B {
    fn run(&mut self, _: Duration) {
        if self.vcc.state.as_logic(1.0) == State::High {
            if !self.powered {
                self.powered = true;
            }
            self.gnd.state = State::Low;

            // check Chip Select (active low)
            if self.cs.state == State::Low {
                // check Write Enable (active low)
                if self.oe.state == State::Low {
                    // IO = Output
                    self.set_io_type(PinType::Output);

                    // display data on IO pins
                    let addr = Pin::read(&[
                        &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                        &self.a7,
                    ]);
                    Pin::write(
                        &mut [
                            &mut self.io0,
                            &mut self.io1,
                            &mut self.io2,
                            &mut self.io3,
                            &mut self.io4,
                            &mut self.io5,
                            &mut self.io6,
                            &mut self.io7,
                        ],
                        self.rom[addr] as usize,
                    );
                } else {
                    self.set_io_type(PinType::Floating);
                }
            } else {
                self.set_io_type(PinType::Floating);
            }
        } else if self.powered {
            self.set_io_type(PinType::Floating);
            self.powered = false;
        }
    }
}

impl ToString for Rom256B {
    fn to_string(&self) -> std::string::String {
        let mut string = String::from(
            "ADR| 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
---+------------------------------------------------",
        );
        for (addr, byte) in self.rom.iter().enumerate() {
            if addr % 16 == 0 {
                string.push_str(&format!("\n {addr:02X}|"));
            }
            string.push_str(&format!(
                "{}{byte:02X}",
                if self.cs.state.as_logic(3.3) == State::Low
                    && Pin::read(&[
                        &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                        &self.a7
                    ]) > 0
                {
                    ">"
                } else {
                    " "
                }
            ));
        }
        string.push('\n');
        string
    }
}
