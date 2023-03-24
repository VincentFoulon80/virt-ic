pub mod assembler;
pub mod opcodes;

pub use assembler::Assembler;
pub use opcodes::{AddressingMode, Opcode};

use crate::{
    chip::{ChipBuilder, ChipRunner, ChipType, ListenerStorage, Pin, PinType},
    generate_chip, impl_listener, State,
};

use bitflags::bitflags;

use super::Reg;

bitflags! {
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatusRegister: u8 {
        /// Negative
        const N = 0b10000000;
        /// Overflow
        const V = 0b01000000;
        /// Break
        const B = 0b00010000;
        /// Decimal
        const D = 0b00001000;
        /// Interrupt Disable
        const I = 0b00000100;
        /// Zero
        const Z = 0b00000010;
        /// Carry
        const C = 0b00000001;
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CpuState {
    Reset,
    ResetCollectHighByte,
    ResetCollectLowByte,
    NmiCollectHighByte,
    NmiCollectLowByte,
    IrqCollectHighByte,
    IrqCollectLowByte,
    Fetch,
    Arg1(Opcode),
    Arg2(Opcode),
    Execute(Opcode, usize),
    Halted,
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Registers {
    pub a: Reg<u8>,
    pub x: Reg<u8>,
    pub y: Reg<u8>,
    pub pc: Reg<u16>,
    pub s: Reg<u8>,
    pub p: StatusRegister,
}

impl ToString for Registers {
    fn to_string(&self) -> std::string::String {
        format!(
            "A={:0X}\tX={:0X}\tY={:0X}\tS={:0X}\tPC={:0X}\tP={}{}{}{}{}{}{}{}",
            *self.a,
            *self.x,
            *self.y,
            *self.s,
            *self.pc,
            if self.p.contains(StatusRegister::N) {
                "N"
            } else {
                "-"
            },
            if self.p.contains(StatusRegister::V) {
                "V"
            } else {
                "-"
            },
            "-",
            if self.p.contains(StatusRegister::B) {
                "B"
            } else {
                "-"
            },
            if self.p.contains(StatusRegister::D) {
                "D"
            } else {
                "-"
            },
            if self.p.contains(StatusRegister::I) {
                "I"
            } else {
                "-"
            },
            if self.p.contains(StatusRegister::Z) {
                "Z"
            } else {
                "-"
            },
            if self.p.contains(StatusRegister::C) {
                "C"
            } else {
                "-"
            },
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CpuEvent {
    Execute { opcode: Opcode },
}

/// https://www.nesdev.org/wiki/CPU_pinout
/// Without the APU part yet
/// Neither the interrupt handling and decimal mode
/// WARNING: Not cycle accurate yet!
///
/// ```txt
///         .--\/--.
///  AD1 <- |01  40| -- +5V
///  AD2 <- |02  39| -> OUT0
/// /RST -> |03  38| -> OUT1
///  A00 <- |04  37| -> OUT2
///  A01 <- |05  36| -> /OE1
///  A02 <- |06  35| -> /OE2
///  A03 <- |07  34| -> R/W
///  A04 <- |08  33| <- /NMI
///  A05 <- |09  32| <- /IRQ
///  A06 <- |10  31| -> M2
///  A07 <- |11  30| <- TST (usually GND)
///  A08 <- |12  29| <- CLK
///  A09 <- |13  28| <> D0
///  A10 <- |14  27| <> D1
///  A11 <- |15  26| <> D2
///  A12 <- |16  25| <> D3
///  A13 <- |17  24| <> D4
///  A14 <- |18  23| <> D5
///  A15 <- |19  22| <> D6
///  GND -- |20  21| <> D7
///         `------'
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Nes6502 {
    powered: bool,
    clock: bool,
    state: CpuState,
    registers: Registers,
    buffer: u16,
    #[serde(skip)]
    listeners: ListenerStorage<Self, CpuEvent>,
    pub vcc: Pin,
    pub gnd: Pin,
    pub rst: Pin,
    pub out0: Pin,
    pub out1: Pin,
    pub out2: Pin,
    pub oe1: Pin,
    pub oe2: Pin,
    pub rw: Pin,
    pub nmi: Pin,
    pub irq: Pin,
    pub m2: Pin,
    pub tst: Pin,
    pub clk: Pin,
    pub ad1: Pin,
    pub ad2: Pin,
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
    pub a13: Pin,
    pub a14: Pin,
    pub a15: Pin,
    pub d0: Pin,
    pub d1: Pin,
    pub d2: Pin,
    pub d3: Pin,
    pub d4: Pin,
    pub d5: Pin,
    pub d6: Pin,
    pub d7: Pin,
}

impl Nes6502 {
    pub const VCC: usize = 40;
    pub const GND: usize = 20;
    pub const RST: usize = 3;
    pub const OUT0: usize = 39;
    pub const OUT1: usize = 38;
    pub const OUT2: usize = 37;
    pub const OE1: usize = 36;
    pub const OE2: usize = 35;
    /// Read/write signal, which is used to indicate operations of the same names. Low is write. R/W stays high/low during the entire read/write cycle.
    pub const RW: usize = 34;
    /// Non-maskable interrupt pin. See the 6502 manual and CPU interrupts for more details.
    pub const NMI: usize = 33;
    /// Interrupt pin. See the 6502 manual and CPU interrupts for more details
    pub const IRQ: usize = 32;
    pub const M2: usize = 31;
    pub const TST: usize = 30;
    pub const CLK: usize = 29;
    pub const AD1: usize = 1;
    pub const AD2: usize = 2;
    pub const A0: usize = 4;
    pub const A1: usize = 5;
    pub const A2: usize = 6;
    pub const A3: usize = 7;
    pub const A4: usize = 8;
    pub const A5: usize = 9;
    pub const A6: usize = 10;
    pub const A7: usize = 11;
    pub const A8: usize = 12;
    pub const A9: usize = 13;
    pub const A10: usize = 14;
    pub const A11: usize = 15;
    pub const A12: usize = 16;
    pub const A13: usize = 17;
    pub const A14: usize = 18;
    pub const A15: usize = 19;
    pub const D0: usize = 28;
    pub const D1: usize = 27;
    pub const D2: usize = 26;
    pub const D3: usize = 25;
    pub const D4: usize = 24;
    pub const D5: usize = 23;
    pub const D6: usize = 22;
    pub const D7: usize = 21;
}

generate_chip!(
    Nes6502,
    vcc: Nes6502::VCC,
    gnd: Nes6502::GND,
    rst: Nes6502::RST,
    out0: Nes6502::OUT0,
    out1: Nes6502::OUT1,
    out2: Nes6502::OUT2,
    oe1: Nes6502::OE1,
    oe2: Nes6502::OE2,
    rw: Nes6502::RW,
    nmi: Nes6502::NMI,
    irq: Nes6502::IRQ,
    m2: Nes6502::M2,
    tst: Nes6502::TST,
    clk: Nes6502::CLK,
    ad1: Nes6502::AD1,
    ad2: Nes6502::AD2,
    a0: Nes6502::A0,
    a1: Nes6502::A1,
    a2: Nes6502::A2,
    a3: Nes6502::A3,
    a4: Nes6502::A4,
    a5: Nes6502::A5,
    a6: Nes6502::A6,
    a7: Nes6502::A7,
    a8: Nes6502::A8,
    a9: Nes6502::A9,
    a10: Nes6502::A10,
    a11: Nes6502::A11,
    a12: Nes6502::A12,
    a13: Nes6502::A13,
    a14: Nes6502::A14,
    a15: Nes6502::A15,
    d0: Nes6502::D0,
    d1: Nes6502::D1,
    d2: Nes6502::D2,
    d3: Nes6502::D3,
    d4: Nes6502::D4,
    d5: Nes6502::D5,
    d6: Nes6502::D6,
    d7: Nes6502::D7
);

impl_listener!(Nes6502: listeners, CpuEvent);

impl ChipBuilder<ChipType> for Nes6502 {
    fn build() -> ChipType {
        ChipType::Nes6502(Box::new(Nes6502 {
            powered: false,
            clock: false,
            state: CpuState::Reset,
            registers: Registers::default(),
            buffer: 0,
            listeners: ListenerStorage::default(),
            vcc: Pin::from(PinType::Input),
            gnd: Pin::from(PinType::Output),
            rst: Pin::from(PinType::Input),
            out0: Pin::from(PinType::Output),
            out1: Pin::from(PinType::Output),
            out2: Pin::from(PinType::Output),
            oe1: Pin::from(PinType::Output),
            oe2: Pin::from(PinType::Output),
            rw: Pin::from(PinType::Output),
            nmi: Pin::from(PinType::Input),
            irq: Pin::from(PinType::Input),
            m2: Pin::from(PinType::Output),
            tst: Pin::from(PinType::Input),
            clk: Pin::from(PinType::Input),
            ad1: Pin::from(PinType::Output),
            ad2: Pin::from(PinType::Output),
            a0: Pin::from(PinType::Output),
            a1: Pin::from(PinType::Output),
            a2: Pin::from(PinType::Output),
            a3: Pin::from(PinType::Output),
            a4: Pin::from(PinType::Output),
            a5: Pin::from(PinType::Output),
            a6: Pin::from(PinType::Output),
            a7: Pin::from(PinType::Output),
            a8: Pin::from(PinType::Output),
            a9: Pin::from(PinType::Output),
            a10: Pin::from(PinType::Output),
            a11: Pin::from(PinType::Output),
            a12: Pin::from(PinType::Output),
            a13: Pin::from(PinType::Output),
            a14: Pin::from(PinType::Output),
            a15: Pin::from(PinType::Output),
            d0: Pin::from(PinType::Floating),
            d1: Pin::from(PinType::Floating),
            d2: Pin::from(PinType::Floating),
            d3: Pin::from(PinType::Floating),
            d4: Pin::from(PinType::Floating),
            d5: Pin::from(PinType::Floating),
            d6: Pin::from(PinType::Floating),
            d7: Pin::from(PinType::Floating),
        }))
    }
}

impl Nes6502 {
    pub fn set_addr(&mut self, addr: u16) {
        Pin::write(
            &mut [
                &mut self.a0,
                &mut self.a1,
                &mut self.a2,
                &mut self.a3,
                &mut self.a4,
                &mut self.a5,
                &mut self.a6,
                &mut self.a7,
                &mut self.a8,
                &mut self.a9,
                &mut self.a10,
                &mut self.a11,
                &mut self.a12,
                &mut self.a13,
                &mut self.a14,
                &mut self.a15,
            ],
            addr as usize,
        );
    }

    fn set_data_type(&mut self, pin_type: PinType) {
        match pin_type {
            PinType::Input => self.rw.state = State::High,
            PinType::Output => self.rw.state = State::Low,
            PinType::Floating => {}
        }
        self.d0.pin_type = pin_type;
        self.d1.pin_type = pin_type;
        self.d2.pin_type = pin_type;
        self.d3.pin_type = pin_type;
        self.d4.pin_type = pin_type;
        self.d5.pin_type = pin_type;
        self.d6.pin_type = pin_type;
        self.d7.pin_type = pin_type;
    }

    pub fn set_data(&mut self, data: u8) {
        Pin::write(
            &mut [
                &mut self.d0,
                &mut self.d1,
                &mut self.d2,
                &mut self.d3,
                &mut self.d4,
                &mut self.d5,
                &mut self.d6,
                &mut self.d7,
            ],
            data as usize,
        );
    }

    pub fn get_data(&self) -> u8 {
        Pin::read_threshold(
            &[
                &self.d0, &self.d1, &self.d2, &self.d3, &self.d4, &self.d5, &self.d6, &self.d7,
            ],
            3.3,
        ) as u8
    }
}

impl ChipRunner for Nes6502 {
    fn run(&mut self, _: std::time::Duration) {
        if self.vcc.state.as_logic(3.3) == State::High {
            if !self.powered {
                self.state = CpuState::Reset;
                self.registers.p = StatusRegister::from_bits_retain(0x34);
                self.registers.a = 0.into();
                self.registers.x = 0.into();
                self.registers.y = 0.into();
                self.registers.s = 0xFD.into();
                self.registers.pc = 0xFFFC.into();

                self.powered = true;
            }

            if self.clock != self.clk.state.as_logic(3.3).into() {
                self.clock = self.clk.state.as_logic(3.3).into();
                self.m2.state = State::from(self.clock);
                if self.clock {
                    match self.state {
                        CpuState::Reset
                        | CpuState::ResetCollectHighByte
                        | CpuState::ResetCollectLowByte
                        | CpuState::NmiCollectHighByte
                        | CpuState::NmiCollectLowByte
                        | CpuState::IrqCollectHighByte
                        | CpuState::IrqCollectLowByte
                        | CpuState::Fetch
                        | CpuState::Arg1(_)
                        | CpuState::Arg2(_) => {
                            self.set_data_type(PinType::Input);
                        }
                        CpuState::Execute(_, _) => {}
                        CpuState::Halted => self.set_data_type(PinType::Floating),
                    }
                } else {
                    match self.state {
                        CpuState::Reset => {
                            self.set_addr(*self.registers.pc);
                            self.registers.pc.inc();
                            self.state = CpuState::ResetCollectHighByte;
                        }
                        CpuState::ResetCollectHighByte => {
                            self.buffer = (self.get_data() as u16) << 8;
                            self.set_addr(*self.registers.pc);
                            self.registers.pc.inc();
                            self.state = CpuState::ResetCollectLowByte;
                        }
                        CpuState::ResetCollectLowByte => {
                            self.buffer = self.buffer.wrapping_add(self.get_data() as u16);
                            self.registers.pc = self.buffer.into();
                            self.set_addr(*self.registers.pc);
                            self.registers.pc.inc();
                            self.state = CpuState::Fetch;
                        }
                        CpuState::NmiCollectHighByte => todo!(),
                        CpuState::NmiCollectLowByte => todo!(),
                        CpuState::IrqCollectHighByte => todo!(),
                        CpuState::IrqCollectLowByte => todo!(),
                        CpuState::Fetch => {
                            let opcode = Opcode::from(self.get_data());
                            if opcode.require_arg1() {
                                self.set_addr(*self.registers.pc);
                                self.registers.pc.inc();
                                self.state = CpuState::Arg1(opcode);
                            } else {
                                self.state = CpuState::Execute(opcode, 0);
                            }
                        }
                        CpuState::Arg1(mut opcode) => {
                            opcode.set_arg1(self.get_data());
                            if opcode.require_arg2() {
                                self.set_addr(*self.registers.pc);
                                self.registers.pc.inc();
                                self.state = CpuState::Arg2(opcode);
                            } else {
                                self.state = CpuState::Execute(opcode, 0);
                            }
                        }
                        CpuState::Arg2(mut opcode) => {
                            opcode.set_arg2(self.get_data());
                            self.state = CpuState::Execute(opcode, 0);
                        }
                        CpuState::Execute(mut opcode, mut step) => {
                            self.trigger_event(CpuEvent::Execute { opcode });
                            if opcode.need_compute() {
                                opcode.compute(self, step);
                                if !opcode.need_compute() {
                                    step = 0;
                                } else {
                                    step += 1;
                                }
                            } else {
                                match opcode {
                                    Opcode::ADC(a) => match a {
                                        AddressingMode::Immediate(i) => self.run_adc(i),
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_adc(self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_adc(self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::AND(a) => match a {
                                        AddressingMode::Immediate(i) => self.run_and(i),
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_and(self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_and(self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::ASL(a) => match a {
                                        AddressingMode::Implicit => {
                                            self.registers.p.set(
                                                StatusRegister::C,
                                                (self.registers.a & 0x80) > 0,
                                            );
                                            self.registers.a <<= 1;
                                            self.set_flags_nz(*self.registers.a);
                                            self.state = CpuState::Fetch;
                                        }
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let mut data = self.get_data();
                                                self.registers
                                                    .p
                                                    .set(StatusRegister::C, (data & 0x80) > 0);
                                                data <<= 1;
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                self.set_flags_nz(data);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let mut data = self.get_data();
                                                self.registers
                                                    .p
                                                    .set(StatusRegister::C, (data & 0x80) > 0);
                                                data <<= 1;
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                self.set_flags_nz(data);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::BIT(a) => match a {
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_bit(self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_bit(self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::BPL(ra) => {
                                        if !self.registers.p.contains(StatusRegister::N) {
                                            self.jump_relative(ra);
                                        }
                                        self.state = CpuState::Fetch;
                                    }
                                    Opcode::BMI(ra) => {
                                        if self.registers.p.contains(StatusRegister::N) {
                                            self.jump_relative(ra);
                                        }
                                        self.state = CpuState::Fetch;
                                    }
                                    Opcode::BVC(ra) => {
                                        if !self.registers.p.contains(StatusRegister::V) {
                                            self.jump_relative(ra);
                                        }
                                        self.state = CpuState::Fetch;
                                    }
                                    Opcode::BVS(ra) => {
                                        if self.registers.p.contains(StatusRegister::V) {
                                            self.jump_relative(ra);
                                        }
                                        self.state = CpuState::Fetch;
                                    }
                                    Opcode::BCC(ra) => {
                                        if !self.registers.p.contains(StatusRegister::C) {
                                            self.jump_relative(ra);
                                        }
                                        self.state = CpuState::Fetch;
                                    }
                                    Opcode::BCS(ra) => {
                                        if self.registers.p.contains(StatusRegister::C) {
                                            self.jump_relative(ra);
                                        }
                                        self.state = CpuState::Fetch;
                                    }
                                    Opcode::BNE(ra) => {
                                        if !self.registers.p.contains(StatusRegister::Z) {
                                            self.jump_relative(ra);
                                        }
                                        self.state = CpuState::Fetch;
                                    }
                                    Opcode::BEQ(ra) => {
                                        if self.registers.p.contains(StatusRegister::Z) {
                                            self.jump_relative(ra);
                                        }
                                        self.state = CpuState::Fetch;
                                    }
                                    Opcode::BRK => todo!(),
                                    Opcode::CMP(a) => match a {
                                        AddressingMode::Immediate(i) => {
                                            self.run_cmp(self.registers.a, i);
                                        }
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_cmp(self.registers.a, self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_cmp(self.registers.a, self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::CPX(a) => match a {
                                        AddressingMode::Immediate(i) => {
                                            self.run_cmp(self.registers.x, i);
                                        }
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_cmp(self.registers.x, self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_cmp(self.registers.x, self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::CPY(a) => match a {
                                        AddressingMode::Immediate(i) => {
                                            self.run_cmp(self.registers.y, i);
                                        }
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_cmp(self.registers.y, self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_cmp(self.registers.y, self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::DEC(a) => match a {
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let data = self.get_data().wrapping_sub(1);
                                                self.set_flags_nz(data);
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let data = self.get_data().wrapping_sub(1);
                                                self.set_flags_nz(data);
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::EOR(a) => match a {
                                        AddressingMode::Immediate(i) => self.run_eor(i),
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_eor(self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_eor(self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::CLC => {
                                        self.registers.p.set(StatusRegister::C, false);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::SEC => {
                                        self.registers.p.set(StatusRegister::C, true);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::CLI => {
                                        self.registers.p.set(StatusRegister::I, false);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::SEI => {
                                        self.registers.p.set(StatusRegister::I, true);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::CLV => {
                                        self.registers.p.set(StatusRegister::V, false);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::INC(a) => match a {
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let data = self.get_data().wrapping_add(1);
                                                self.set_flags_nz(data);
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let data = self.get_data().wrapping_add(1);
                                                self.set_flags_nz(data);
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::JMP(AddressingMode::Absolute(a)) => {
                                        self.registers.pc = a.into();
                                        self.state = CpuState::Fetch;
                                    }
                                    Opcode::JMP(_) => unreachable!(),
                                    Opcode::JSR(AddressingMode::Absolute(a)) => {
                                        if step == 0 {
                                            let alt_pc = (*self.registers.pc).wrapping_sub(1);
                                            self.push_stack((alt_pc >> 8) as u8);
                                            step += 1;
                                        } else if step == 1 {
                                            let alt_pc = (*self.registers.pc).wrapping_sub(1);
                                            self.push_stack(alt_pc as u8);
                                            step += 1;
                                        } else {
                                            self.registers.pc = a.into();
                                            self.state = CpuState::Fetch;
                                        }
                                    }
                                    Opcode::JSR(_) => unreachable!(),
                                    Opcode::LDA(a) => match a {
                                        AddressingMode::Immediate(i) => {
                                            self.run_lda(i);
                                        }
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_lda(self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_lda(self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::LDX(a) => match a {
                                        AddressingMode::Immediate(i) => {
                                            self.run_ldx(i);
                                        }
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_ldx(self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_ldx(self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::LDY(a) => match a {
                                        AddressingMode::Immediate(i) => {
                                            self.run_ldy(i);
                                        }
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_ldy(self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_ldy(self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::LSR(a) => match a {
                                        AddressingMode::Implicit => {
                                            self.registers.p.set(
                                                StatusRegister::C,
                                                (self.registers.a & 0x01) > 0,
                                            );
                                            self.registers.a >>= 1;
                                            self.set_flags_nz(*self.registers.a);
                                            self.state = CpuState::Fetch;
                                        }
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let mut data = self.get_data();
                                                self.registers
                                                    .p
                                                    .set(StatusRegister::C, (data & 0x01) > 0);
                                                data >>= 1;
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                self.set_flags_nz(data);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let mut data = self.get_data();
                                                self.registers
                                                    .p
                                                    .set(StatusRegister::C, (data & 0x01) > 0);
                                                data >>= 1;
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                self.set_flags_nz(data);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::NOP => self.state = CpuState::Fetch,
                                    Opcode::ORA(a) => match a {
                                        AddressingMode::Immediate(i) => self.run_ora(i),
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_ora(self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_ora(self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::TAX => {
                                        self.registers.x = self.registers.a;
                                        self.set_flags_nz(*self.registers.x);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::TXA => {
                                        self.registers.a = self.registers.x;
                                        self.set_flags_nz(*self.registers.a);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::DEX => {
                                        self.registers.x.dec();
                                        self.set_flags_nz(*self.registers.x);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::INX => {
                                        self.registers.x.inc();
                                        self.set_flags_nz(*self.registers.x);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::TAY => {
                                        self.registers.y = self.registers.a;
                                        self.set_flags_nz(*self.registers.y);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::TYA => {
                                        self.registers.a = self.registers.y;
                                        self.set_flags_nz(*self.registers.a);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::DEY => {
                                        self.registers.y.dec();
                                        self.set_flags_nz(*self.registers.y);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::INY => {
                                        self.registers.y.inc();
                                        self.set_flags_nz(*self.registers.y);
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::ROL(a) => match a {
                                        AddressingMode::Implicit => {
                                            let old_carry =
                                                self.registers.p.contains(StatusRegister::C) as u8;
                                            self.registers.p.set(
                                                StatusRegister::C,
                                                (self.registers.a & 0x80) > 0,
                                            );
                                            self.registers.a <<= 1;
                                            self.registers.a += old_carry;
                                            self.set_flags_nz(*self.registers.a);
                                            self.state = CpuState::Fetch;
                                        }
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let mut data = self.get_data();
                                                let old_carry =
                                                    self.registers.p.contains(StatusRegister::C)
                                                        as u8;
                                                self.registers
                                                    .p
                                                    .set(StatusRegister::C, (data & 0x80) > 0);
                                                data <<= 1;
                                                data += old_carry;
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                self.set_flags_nz(data);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let mut data = self.get_data();
                                                let old_carry =
                                                    self.registers.p.contains(StatusRegister::C)
                                                        as u8;
                                                self.registers
                                                    .p
                                                    .set(StatusRegister::C, (data & 0x80) > 0);
                                                data <<= 1;
                                                data += old_carry;
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                self.set_flags_nz(data);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::ROR(a) => match a {
                                        AddressingMode::Implicit => {
                                            let old_carry =
                                                self.registers.p.contains(StatusRegister::C) as u8;
                                            self.registers.p.set(
                                                StatusRegister::C,
                                                (self.registers.a & 0x01) > 0,
                                            );
                                            self.registers.a >>= 1;
                                            self.registers.a += old_carry << 7;
                                            self.set_flags_nz(*self.registers.a);
                                            self.state = CpuState::Fetch;
                                        }
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let mut data = self.get_data();
                                                let old_carry =
                                                    self.registers.p.contains(StatusRegister::C)
                                                        as u8;
                                                self.registers
                                                    .p
                                                    .set(StatusRegister::C, (data & 0x01) > 0);
                                                data >>= 1;
                                                data += old_carry << 7;
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                self.set_flags_nz(data);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else if step == 1 {
                                                let mut data = self.get_data();
                                                let old_carry =
                                                    self.registers.p.contains(StatusRegister::C)
                                                        as u8;
                                                self.registers
                                                    .p
                                                    .set(StatusRegister::C, (data & 0x01) > 0);
                                                data <<= 1;
                                                data += old_carry << 7;
                                                self.set_data(data);
                                                self.set_data_type(PinType::Output);
                                                self.set_flags_nz(data);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::RTI => todo!(),
                                    Opcode::RTS => {
                                        if step == 0 {
                                            self.buffer = 0;
                                            self.pop_stack_prepare();
                                            step += 1;
                                        } else if step == 1 {
                                            self.buffer = self.get_data() as u16;
                                            self.pop_stack_prepare();
                                            step += 1;
                                        } else {
                                            self.buffer += (self.get_data() as u16) << 8;
                                            self.registers.pc = self.buffer.wrapping_add(1).into();
                                            self.state = CpuState::Fetch;
                                        }
                                    }
                                    Opcode::SBC(a) => match a {
                                        AddressingMode::Immediate(i) => self.run_sbc(i),
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.set_addr(z as u16);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_sbc(self.get_data());
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                self.set_addr(a);
                                                self.set_data_type(PinType::Input);
                                                step += 1;
                                            } else {
                                                self.run_sbc(self.get_data());
                                            }
                                        }
                                        _ => unreachable!(),
                                    },

                                    Opcode::TXS => {
                                        self.registers.s = self.registers.x;
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::TSX => {
                                        self.registers.x = self.registers.s;
                                        self.state = CpuState::Fetch
                                    }
                                    Opcode::PHA => {
                                        if step == 0 {
                                            self.push_stack(*self.registers.a);
                                            step += 1;
                                        } else {
                                            self.state = CpuState::Fetch;
                                        }
                                    }
                                    Opcode::PLA => {
                                        if step == 0 {
                                            self.pop_stack_prepare();
                                            step += 1;
                                        } else {
                                            self.registers.a = self.get_data().into();
                                        }
                                    }
                                    Opcode::PHP => {
                                        if step == 0 {
                                            self.push_stack(self.registers.p.bits());
                                            step += 1;
                                        } else {
                                            self.state = CpuState::Fetch;
                                        }
                                    }
                                    Opcode::PLP => {
                                        if step == 0 {
                                            self.pop_stack_prepare();
                                            step += 1;
                                        } else {
                                            self.registers.p =
                                                StatusRegister::from_bits_retain(self.get_data());
                                        }
                                    }
                                    Opcode::STA(a) => match a {
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.run_st(*self.registers.a, z as u16);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                step += 1;
                                            } else if step == 1 {
                                                self.run_st(*self.registers.a, a);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::STX(a) => match a {
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.run_st(*self.registers.x, z as u16);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                step += 1;
                                            } else if step == 1 {
                                                self.run_st(*self.registers.x, a);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                    Opcode::STY(a) => match a {
                                        AddressingMode::ZeroPage(z) => {
                                            if step == 0 {
                                                self.run_st(*self.registers.y, z as u16);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        AddressingMode::Absolute(a) => {
                                            if step == 0 {
                                                step += 1;
                                            } else if step == 1 {
                                                self.run_st(*self.registers.y, a);
                                                step += 1;
                                            } else {
                                                self.state = CpuState::Fetch;
                                            }
                                        }
                                        _ => unreachable!(),
                                    },
                                }
                            }
                            if matches!(self.state, CpuState::Execute(_, _)) {
                                self.state = CpuState::Execute(opcode, step);
                            }
                            if matches!(self.state, CpuState::Fetch) {
                                self.set_addr(*self.registers.pc);
                                self.registers.pc.inc();
                            }
                        }
                        CpuState::Halted => {}
                    }
                }
            }
        } else if self.powered {
            self.state = CpuState::Halted;
            self.powered = false;
        }
    }
}

impl Nes6502 {
    fn set_flags_nz(&mut self, val: u8) {
        self.registers.p.set(StatusRegister::Z, val == 0);
        self.registers.p.set(StatusRegister::N, (val & 0x80) > 0);
    }

    fn jump_relative(&mut self, val: i8) {
        self.registers.pc = ((*self.registers.pc as i32 + val as i32) as u16).into()
    }

    fn push_stack(&mut self, val: u8) {
        self.run_st(val, 0x100 + *self.registers.s as u16);
        self.registers.s.dec();
    }

    fn pop_stack_prepare(&mut self) {
        self.registers.s.inc();
        self.set_addr(0x100 + *self.registers.s as u16);
        self.set_data_type(PinType::Input);
    }

    fn run_adc(&mut self, val: u8) {
        let rhs = val.wrapping_add(self.registers.p.contains(StatusRegister::C) as u8);
        let sum = *self.registers.a as u16 + rhs as u16;

        self.registers.p.set(StatusRegister::C, sum > 0xFF);
        let sum: Reg<u8> = (sum as u8).into();

        self.set_flags_nz(*sum);
        self.registers.p.set(
            StatusRegister::V,
            (!(*self.registers.a ^ val) & (*self.registers.a ^ *sum) & 0x80) > 0,
        );
        self.registers.a = sum;
        self.state = CpuState::Fetch;
    }

    fn run_and(&mut self, val: u8) {
        self.registers.a &= val;

        self.set_flags_nz(*self.registers.a);

        self.state = CpuState::Fetch;
    }

    fn run_bit(&mut self, val: u8) {
        self.registers
            .p
            .set(StatusRegister::Z, (*self.registers.a & val) == 0);
        self.registers.p.set(StatusRegister::N, (val & 0x80) > 0);
        self.registers.p.set(StatusRegister::V, (val & 0x40) > 0);

        self.state = CpuState::Fetch;
    }

    fn run_cmp(&mut self, base: Reg<u8>, val: u8) {
        let val = !val;
        let rhs = val.wrapping_add(self.registers.p.contains(StatusRegister::C) as u8);
        let sum = *base as u16 + rhs as u16;

        self.registers.p.set(StatusRegister::C, sum > 0xFF);
        let sum: Reg<u8> = (sum as u8).into();

        self.set_flags_nz(*sum);
        self.registers.p.set(
            StatusRegister::V,
            (!(*base ^ val) & (*base ^ *sum) & 0x80) > 0,
        );
        self.state = CpuState::Fetch;
    }

    fn run_eor(&mut self, val: u8) {
        self.registers.a ^= val;

        self.set_flags_nz(*self.registers.a);

        self.state = CpuState::Fetch;
    }

    fn run_lda(&mut self, val: u8) {
        self.registers.a = val.into();
        self.set_flags_nz(*self.registers.a);
        self.state = CpuState::Fetch;
    }
    fn run_ldx(&mut self, val: u8) {
        self.registers.x = val.into();
        self.set_flags_nz(*self.registers.x);
        self.state = CpuState::Fetch;
    }
    fn run_ldy(&mut self, val: u8) {
        self.registers.y = val.into();
        self.set_flags_nz(*self.registers.y);
        self.state = CpuState::Fetch;
    }

    fn run_ora(&mut self, val: u8) {
        self.registers.a |= val;

        self.set_flags_nz(*self.registers.a);

        self.state = CpuState::Fetch;
    }

    fn run_sbc(&mut self, val: u8) {
        self.run_adc(!val)
    }

    fn run_st(&mut self, val: u8, addr: u16) {
        self.set_addr(addr);
        self.set_data(val);
        self.set_data_type(PinType::Output);
    }
}

impl ToString for Nes6502 {
    fn to_string(&self) -> std::string::String {
        format!("state={:?}\n{}", self.state, self.registers.to_string())
    }
}
