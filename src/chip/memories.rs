use std::time::Duration;

use rand::random;

use crate::{generate_chip, impl_listener, State};

use super::{ChipBuilder, ChipRunner, ChipType, ListenerStorage, Pin, PinType};

#[derive(Debug, Clone, Copy)]
pub enum MemoryEvent {
    WriteByte { addr: usize, byte: u8 },
    ReadByte { addr: usize, byte: u8 },
}

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
    #[serde(skip)]
    listeners: ListenerStorage<Self, MemoryEvent>,
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

impl_listener!(Ram256B: listeners, MemoryEvent);

impl ChipBuilder<ChipType> for Ram256B {
    fn build() -> ChipType {
        ChipType::Ram256B(Ram256B {
            powered: false,
            listeners: ListenerStorage::default(),
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
                    let addr = Pin::read_threshold(
                        &[
                            &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                            &self.a7,
                        ],
                        3.3,
                    );
                    let byte = Pin::read_threshold(
                        &[
                            &self.io0, &self.io1, &self.io2, &self.io3, &self.io4, &self.io5,
                            &self.io6, &self.io7,
                        ],
                        3.3,
                    ) as u8;
                    self.ram[addr] = byte;
                    self.trigger_event(MemoryEvent::WriteByte { addr, byte })
                } else if self.oe.state == State::Low {
                    // IO = Output
                    self.set_io_type(PinType::Output);

                    // display data on IO pins
                    let addr = Pin::read_threshold(
                        &[
                            &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                            &self.a7,
                        ],
                        3.3,
                    );
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
                    self.trigger_event(MemoryEvent::ReadByte {
                        addr,
                        byte: self.ram[addr],
                    })
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
            "ADR| 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F\n---+------------------------------------------------",
        );
        for (addr, byte) in self.ram.iter().enumerate() {
            if addr % 16 == 0 {
                string.push_str(&format!("\n {addr:02X}|"));
            }
            string.push_str(&format!(
                "{}{byte:02X}",
                if self.cs.state.as_logic(3.3) == State::Low
                    && Pin::read_threshold(
                        &[
                            &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                            &self.a7
                        ],
                        3.3
                    ) == addr
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

/// # A 8KB RAM chip
///
/// # Diagram
/// CS: Chip Select (active low)
/// WE: Write Enable (active low)
/// OE: Output Enable (active low)
/// A0-12: Addresses
/// IO0-7: Input/Output
/// ```
///        ---__---
///  !CS --|1   26|-- VCC
///  !WE --|2   25|-- IO7
///  !OE --|3   24|-- IO6
///   A0 --|4   23|-- IO5
///   A1 --|5   22|-- IO4
///   A2 --|6   21|-- IO3
///   A3 --|7   20|-- IO2
///   A4 --|8   19|-- IO1
///   A5 --|9   18|-- IO0
///   A6 --|10  17|-- A12
///   A7 --|11  16|-- A11
///   A8 --|12  15|-- A10
///  GND --|13  14|-- A9
///        --------
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ram8KB {
    powered: bool,
    #[serde(skip)]
    listeners: ListenerStorage<Self, MemoryEvent>,
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
    pub a8: Pin,
    pub a9: Pin,
    pub a10: Pin,
    pub a11: Pin,
    pub a12: Pin,
    pub io0: Pin,
    pub io1: Pin,
    pub io2: Pin,
    pub io3: Pin,
    pub io4: Pin,
    pub io5: Pin,
    pub io6: Pin,
    pub io7: Pin,
}

impl Ram8KB {
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
    pub const A7: usize = 11;
    pub const A8: usize = 12;
    pub const A9: usize = 14;
    pub const A10: usize = 15;
    pub const A11: usize = 16;
    pub const A12: usize = 17;
    pub const IO0: usize = 18;
    pub const IO1: usize = 19;
    pub const IO2: usize = 20;
    pub const IO3: usize = 21;
    pub const IO4: usize = 22;
    pub const IO5: usize = 23;
    pub const IO6: usize = 24;
    pub const IO7: usize = 25;
    pub const VCC: usize = 26;
    pub const GND: usize = 13;

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
    Ram8KB,
    cs: Ram8KB::CS,
    we: Ram8KB::WE,
    oe: Ram8KB::OE,
    a0: Ram8KB::A0,
    a1: Ram8KB::A1,
    a2: Ram8KB::A2,
    a3: Ram8KB::A3,
    a4: Ram8KB::A4,
    a5: Ram8KB::A5,
    a6: Ram8KB::A6,
    a7: Ram8KB::A7,
    a8: Ram8KB::A8,
    a9: Ram8KB::A9,
    a10: Ram8KB::A10,
    a11: Ram8KB::A11,
    a12: Ram8KB::A12,
    io0: Ram8KB::IO0,
    io1: Ram8KB::IO1,
    io2: Ram8KB::IO2,
    io3: Ram8KB::IO3,
    io4: Ram8KB::IO4,
    io5: Ram8KB::IO5,
    io6: Ram8KB::IO6,
    io7: Ram8KB::IO7,
    vcc: Ram8KB::VCC,
    gnd: Ram8KB::GND
);

impl_listener!(Ram8KB: listeners, MemoryEvent);

impl ChipBuilder<ChipType> for Ram8KB {
    fn build() -> ChipType {
        ChipType::Ram8KB(Ram8KB {
            powered: false,
            listeners: ListenerStorage::default(),
            ram: Vec::from([0; 8192]),
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
            a8: Pin::from(PinType::Input),
            a9: Pin::from(PinType::Input),
            a10: Pin::from(PinType::Input),
            a11: Pin::from(PinType::Input),
            a12: Pin::from(PinType::Input),
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

impl ChipRunner for Ram8KB {
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
                    let addr = Pin::read_threshold(
                        &[
                            &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                            &self.a7, &self.a8, &self.a9, &self.a10, &self.a11, &self.a12,
                        ],
                        3.3,
                    );
                    let byte = Pin::read_threshold(
                        &[
                            &self.io0, &self.io1, &self.io2, &self.io3, &self.io4, &self.io5,
                            &self.io6, &self.io7,
                        ],
                        3.3,
                    ) as u8;
                    self.ram[addr] = byte;
                    self.trigger_event(MemoryEvent::WriteByte { addr, byte });
                } else if self.oe.state == State::Low {
                    // IO = Output
                    self.set_io_type(PinType::Output);

                    // display data on IO pins
                    let addr = Pin::read_threshold(
                        &[
                            &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                            &self.a7, &self.a8, &self.a9, &self.a10, &self.a11, &self.a12,
                        ],
                        3.3,
                    );
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
                    self.trigger_event(MemoryEvent::ReadByte {
                        addr,
                        byte: self.ram[addr],
                    })
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

impl ToString for Ram8KB {
    fn to_string(&self) -> std::string::String {
        let mut string = String::from(
            "  ADR| 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F\n-----+------------------------------------------------",
        );
        for (addr, byte) in self.ram.iter().enumerate() {
            if addr % 16 == 0 {
                string.push_str(&format!("\n {addr:04X}|"));
            }
            string.push_str(&format!(
                "{}{byte:02X}",
                if self.cs.state.as_logic(3.3) == State::Low
                    && Pin::read_threshold(
                        &[
                            &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                            &self.a7, &self.a8, &self.a9, &self.a10, &self.a11, &self.a12,
                        ],
                        3.3
                    ) == addr
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
    #[serde(skip)]
    listeners: ListenerStorage<Self, MemoryEvent>,
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

    pub fn set_data(mut self, data: &[u8]) -> Self {
        self.rom = Vec::from(data);
        self.rom.resize(256, 0);
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

impl_listener!(Rom256B: listeners, MemoryEvent);

impl ChipBuilder<Rom256B> for Rom256B {
    fn build() -> Rom256B {
        Rom256B {
            powered: false,
            listeners: ListenerStorage::default(),
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
                // check Output Enable (active low)
                if self.oe.state == State::Low {
                    // IO = Output
                    self.set_io_type(PinType::Output);

                    // display data on IO pins
                    let addr = Pin::read_threshold(
                        &[
                            &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                            &self.a7,
                        ],
                        3.3,
                    );
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
                    self.trigger_event(MemoryEvent::ReadByte {
                        addr,
                        byte: self.rom[addr],
                    })
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
            "ADR| 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F\n---+------------------------------------------------",
        );
        for (addr, byte) in self.rom.iter().enumerate() {
            if addr % 16 == 0 {
                string.push_str(&format!("\n {addr:02X}|"));
            }
            string.push_str(&format!(
                "{}{byte:02X}",
                if self.cs.state.as_logic(3.3) == State::Low
                    && Pin::read_threshold(
                        &[
                            &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                            &self.a7
                        ],
                        3.3
                    ) > 0
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

/// # A 8KB ROM chip
///
/// # Diagram
/// CS: Chip Select (active low)
/// WE: Write Enable (active low)
/// OE: Output Enable (active low)
/// A0-12: Addresses
/// IO0-7: Input/Output
/// ```
///         ---__---
///   !CS --|1   26|-- VCC
/// UNUSED--|2   25|-- IO7
///   !OE --|3   24|-- IO6
///    A0 --|4   23|-- IO5
///    A1 --|5   22|-- IO4
///    A2 --|6   21|-- IO3
///    A3 --|7   20|-- IO2
///    A4 --|8   19|-- IO1
///    A5 --|9   18|-- IO0
///    A6 --|10  17|-- A12
///    A7 --|11  16|-- A11
///    A8 --|12  15|-- A10
///   GND --|13  14|-- A9
///         --------
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rom8KB {
    powered: bool,
    #[serde(skip)]
    listeners: ListenerStorage<Self, MemoryEvent>,
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
    pub a8: Pin,
    pub a9: Pin,
    pub a10: Pin,
    pub a11: Pin,
    pub a12: Pin,
    pub io0: Pin,
    pub io1: Pin,
    pub io2: Pin,
    pub io3: Pin,
    pub io4: Pin,
    pub io5: Pin,
    pub io6: Pin,
    pub io7: Pin,
}

impl Rom8KB {
    pub const CS: usize = 1;
    pub const OE: usize = 3;
    pub const A0: usize = 4;
    pub const A1: usize = 5;
    pub const A2: usize = 6;
    pub const A3: usize = 7;
    pub const A4: usize = 8;
    pub const A5: usize = 9;
    pub const A6: usize = 10;
    pub const A7: usize = 11;
    pub const A8: usize = 12;
    pub const A9: usize = 14;
    pub const A10: usize = 15;
    pub const A11: usize = 16;
    pub const A12: usize = 17;
    pub const IO0: usize = 18;
    pub const IO1: usize = 19;
    pub const IO2: usize = 20;
    pub const IO3: usize = 21;
    pub const IO4: usize = 22;
    pub const IO5: usize = 23;
    pub const IO6: usize = 24;
    pub const IO7: usize = 25;
    pub const VCC: usize = 26;
    pub const GND: usize = 13;

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

    pub fn set_data(mut self, data: &[u8]) -> Self {
        self.rom = Vec::from(data);
        self.rom.resize(8192, 0);
        self
    }
}

generate_chip!(
    Rom8KB,
    cs: Rom8KB::CS,
    oe: Rom8KB::OE,
    a0: Rom8KB::A0,
    a1: Rom8KB::A1,
    a2: Rom8KB::A2,
    a3: Rom8KB::A3,
    a4: Rom8KB::A4,
    a5: Rom8KB::A5,
    a6: Rom8KB::A6,
    a7: Rom8KB::A7,
    a8: Rom8KB::A8,
    a9: Rom8KB::A9,
    a10: Rom8KB::A10,
    a11: Rom8KB::A11,
    a12: Rom8KB::A12,
    io0: Rom8KB::IO0,
    io1: Rom8KB::IO1,
    io2: Rom8KB::IO2,
    io3: Rom8KB::IO3,
    io4: Rom8KB::IO4,
    io5: Rom8KB::IO5,
    io6: Rom8KB::IO6,
    io7: Rom8KB::IO7,
    vcc: Rom8KB::VCC,
    gnd: Rom8KB::GND
);

impl_listener!(Rom8KB: listeners, MemoryEvent);

impl ChipBuilder<Rom8KB> for Rom8KB {
    fn build() -> Rom8KB {
        Rom8KB {
            powered: false,
            listeners: ListenerStorage::default(),
            rom: Vec::from([0; 8192]),
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
            a8: Pin::from(PinType::Input),
            a9: Pin::from(PinType::Input),
            a10: Pin::from(PinType::Input),
            a11: Pin::from(PinType::Input),
            a12: Pin::from(PinType::Input),
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

impl From<Rom8KB> for ChipType {
    fn from(value: Rom8KB) -> Self {
        ChipType::Rom8KB(value)
    }
}

impl ChipRunner for Rom8KB {
    fn run(&mut self, _: Duration) {
        if self.vcc.state.as_logic(1.0) == State::High {
            if !self.powered {
                self.powered = true;
            }
            self.gnd.state = State::Low;

            // check Chip Select (active low)
            if self.cs.state == State::Low {
                // check Output Enable (active low)
                if self.oe.state == State::Low {
                    // IO = Output
                    self.set_io_type(PinType::Output);

                    // display data on IO pins
                    let addr = Pin::read_threshold(
                        &[
                            &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                            &self.a7, &self.a8, &self.a9, &self.a10, &self.a11, &self.a12,
                        ],
                        3.3,
                    );
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
                    self.trigger_event(MemoryEvent::ReadByte {
                        addr,
                        byte: self.rom[addr],
                    });
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

impl ToString for Rom8KB {
    fn to_string(&self) -> std::string::String {
        let mut string = String::from(
            "  ADR| 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F\n-----+------------------------------------------------",
        );
        for (addr, byte) in self.rom.iter().enumerate() {
            if addr % 16 == 0 {
                string.push_str(&format!("\n {addr:04X}|"));
            }
            string.push_str(&format!(
                "{}{byte:02X}",
                if self.cs.state.as_logic(3.3) == State::Low
                    && Pin::read_threshold(
                        &[
                            &self.a0, &self.a1, &self.a2, &self.a3, &self.a4, &self.a5, &self.a6,
                            &self.a7, &self.a8, &self.a9, &self.a10, &self.a11, &self.a12,
                        ],
                        3.3
                    ) == addr
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
