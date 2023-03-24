use crate::chip::PinType;

use super::Nes6502;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AddressingMode {
    Implicit,
    /// immediate value
    Immediate(u8),
    /// Zero page address $00zz
    ZeroPage(u8),
    /// Absolute address $hhll
    Absolute(u16),
    /// Indirect adressing ($hhll)
    Indirect(u16),
    /// d,x
    /// val = PEEK((arg + X) % 256)
    ZeroPageIndexedX(u8),
    /// d,y
    /// val = PEEK((arg + Y) % 256)
    ZeroPageIndexedY(u8),
    /// a,x
    /// val = PEEK(arg + X)
    AbsoluteIndexedX(u16),
    /// a,y
    /// val = PEEK(arg + Y)
    AbsoluteIndexedY(u16),
    /// (d,x)
    /// val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256)
    IndexedIndirect(u8),
    /// (d),y
    /// val = PEEK(PEEK(arg) + PEEK((arg + 1) % 256) * 256 + Y)
    IndirectIndexed(u8),
}

impl AddressingMode {
    fn set_arg1(&mut self, arg: u8) {
        match self {
            AddressingMode::Immediate(a)
            | AddressingMode::ZeroPage(a)
            | AddressingMode::ZeroPageIndexedX(a)
            | AddressingMode::ZeroPageIndexedY(a)
            | AddressingMode::IndexedIndirect(a)
            | AddressingMode::IndirectIndexed(a) => {
                *a = arg;
            }
            AddressingMode::Absolute(a)
            | AddressingMode::Indirect(a)
            | AddressingMode::AbsoluteIndexedX(a)
            | AddressingMode::AbsoluteIndexedY(a) => {
                *a = arg as u16;
            }
            AddressingMode::Implicit => {}
        }
    }
    fn set_arg2(&mut self, arg: u8) {
        match self {
            AddressingMode::Absolute(a)
            | AddressingMode::Indirect(a)
            | AddressingMode::AbsoluteIndexedX(a)
            | AddressingMode::AbsoluteIndexedY(a) => {
                *a += (arg as u16) << 8;
            }
            _ => {}
        }
    }
    fn require_arg1(&self) -> bool {
        matches!(
            self,
            AddressingMode::Immediate(_)
                | AddressingMode::ZeroPage(_)
                | AddressingMode::Absolute(_)
                | AddressingMode::Indirect(_)
                | AddressingMode::ZeroPageIndexedX(_)
                | AddressingMode::ZeroPageIndexedY(_)
                | AddressingMode::AbsoluteIndexedX(_)
                | AddressingMode::AbsoluteIndexedY(_)
                | AddressingMode::IndexedIndirect(_)
                | AddressingMode::IndirectIndexed(_)
        )
    }

    fn require_arg2(&self) -> bool {
        matches!(
            self,
            AddressingMode::Absolute(_)
                | AddressingMode::Indirect(_)
                | AddressingMode::AbsoluteIndexedX(_)
                | AddressingMode::AbsoluteIndexedY(_)
        )
    }
    fn need_compute(&self) -> bool {
        matches!(
            self,
            AddressingMode::Indirect(_)
                | AddressingMode::ZeroPageIndexedX(_)
                | AddressingMode::ZeroPageIndexedY(_)
                | AddressingMode::AbsoluteIndexedX(_)
                | AddressingMode::AbsoluteIndexedY(_)
                | AddressingMode::IndexedIndirect(_)
                | AddressingMode::IndirectIndexed(_)
        )
    }
    fn compute(&self, cpu: &mut Nes6502, step: usize) -> Self {
        match self {
            AddressingMode::Indirect(a) => {
                if step == 0 {
                    cpu.set_addr(*a);
                    cpu.set_data_type(PinType::Input);
                    cpu.buffer = 0;
                    AddressingMode::Indirect(a.wrapping_add(1))
                } else if step == 1 {
                    cpu.buffer = (cpu.get_data() as u16) << 8;
                    *self
                } else {
                    cpu.buffer = cpu.buffer.wrapping_add(cpu.get_data() as u16);
                    AddressingMode::Absolute(cpu.buffer)
                }
            }
            AddressingMode::ZeroPageIndexedX(z) => {
                AddressingMode::ZeroPage(*(cpu.registers.x + *z))
            }
            AddressingMode::ZeroPageIndexedY(z) => {
                AddressingMode::ZeroPage(*(cpu.registers.y + *z))
            }
            AddressingMode::AbsoluteIndexedX(a) => {
                AddressingMode::Absolute(a + *cpu.registers.x as u16)
            }
            AddressingMode::AbsoluteIndexedY(a) => {
                AddressingMode::Absolute(a + *cpu.registers.y as u16)
            }
            AddressingMode::IndexedIndirect(_) => todo!(),
            AddressingMode::IndirectIndexed(_) => todo!(),
            _ => *self,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Opcode {
    ADC(AddressingMode),
    AND(AddressingMode),
    ASL(AddressingMode),
    BIT(AddressingMode),
    BPL(i8),
    BMI(i8),
    BVC(i8),
    BVS(i8),
    BCC(i8),
    BCS(i8),
    BNE(i8),
    BEQ(i8),
    BRK,
    CMP(AddressingMode),
    CPX(AddressingMode),
    CPY(AddressingMode),
    DEC(AddressingMode),
    EOR(AddressingMode),
    CLC,
    SEC,
    CLI,
    SEI,
    CLV,
    // CLD,
    // SED,
    INC(AddressingMode),
    JMP(AddressingMode),
    JSR(AddressingMode),
    LDA(AddressingMode),
    LDX(AddressingMode),
    LDY(AddressingMode),
    LSR(AddressingMode),
    NOP,
    ORA(AddressingMode),
    TAX,
    TXA,
    DEX,
    INX,
    TAY,
    TYA,
    DEY,
    INY,
    ROL(AddressingMode),
    ROR(AddressingMode),
    RTI,
    RTS,
    SBC(AddressingMode),
    STA(AddressingMode),
    TXS,
    TSX,
    PHA,
    PLA,
    PHP,
    PLP,
    STX(AddressingMode),
    STY(AddressingMode),
}

impl Opcode {
    pub fn set_arg1(&mut self, arg: u8) {
        match self {
            Opcode::LDA(a)
            | Opcode::STA(a)
            | Opcode::AND(a)
            | Opcode::ADC(a)
            | Opcode::SBC(a)
            | Opcode::ASL(a)
            | Opcode::BIT(a)
            | Opcode::CMP(a)
            | Opcode::CPX(a)
            | Opcode::CPY(a)
            | Opcode::DEC(a)
            | Opcode::EOR(a)
            | Opcode::INC(a)
            | Opcode::JMP(a)
            | Opcode::JSR(a)
            | Opcode::LDX(a)
            | Opcode::LDY(a)
            | Opcode::LSR(a)
            | Opcode::ORA(a)
            | Opcode::ROL(a)
            | Opcode::ROR(a)
            | Opcode::STX(a)
            | Opcode::STY(a) => a.set_arg1(arg),
            Opcode::BPL(a)
            | Opcode::BMI(a)
            | Opcode::BVC(a)
            | Opcode::BVS(a)
            | Opcode::BCC(a)
            | Opcode::BCS(a)
            | Opcode::BNE(a)
            | Opcode::BEQ(a) => *a = arg as i8,
            Opcode::NOP
            | Opcode::CLC
            | Opcode::SEC
            | Opcode::CLI
            | Opcode::SEI
            | Opcode::INY
            | Opcode::INX
            | Opcode::DEY
            | Opcode::DEX
            | Opcode::TAY
            | Opcode::TYA
            | Opcode::TAX
            | Opcode::TXA
            | Opcode::TXS
            | Opcode::TSX
            | Opcode::PHA
            | Opcode::PLA
            | Opcode::PHP
            | Opcode::PLP
            | Opcode::BRK
            | Opcode::CLV
            | Opcode::RTI
            | Opcode::RTS => {}
        }
    }
    pub fn set_arg2(&mut self, arg: u8) {
        match self {
            Opcode::LDA(a)
            | Opcode::STA(a)
            | Opcode::AND(a)
            | Opcode::ADC(a)
            | Opcode::SBC(a)
            | Opcode::ASL(a)
            | Opcode::BIT(a)
            | Opcode::CMP(a)
            | Opcode::CPX(a)
            | Opcode::CPY(a)
            | Opcode::DEC(a)
            | Opcode::EOR(a)
            | Opcode::INC(a)
            | Opcode::JMP(a)
            | Opcode::JSR(a)
            | Opcode::LDX(a)
            | Opcode::LDY(a)
            | Opcode::LSR(a)
            | Opcode::ORA(a)
            | Opcode::ROL(a)
            | Opcode::ROR(a)
            | Opcode::STX(a)
            | Opcode::STY(a) => a.set_arg2(arg),
            Opcode::NOP
            | Opcode::CLC
            | Opcode::SEC
            | Opcode::CLI
            | Opcode::SEI
            | Opcode::INY
            | Opcode::INX
            | Opcode::DEY
            | Opcode::DEX
            | Opcode::TAY
            | Opcode::TYA
            | Opcode::TAX
            | Opcode::TXA
            | Opcode::TXS
            | Opcode::TSX
            | Opcode::PHA
            | Opcode::PLA
            | Opcode::PHP
            | Opcode::PLP
            | Opcode::BRK
            | Opcode::CLV
            | Opcode::RTI
            | Opcode::RTS
            | Opcode::BPL(_)
            | Opcode::BMI(_)
            | Opcode::BVC(_)
            | Opcode::BVS(_)
            | Opcode::BCC(_)
            | Opcode::BCS(_)
            | Opcode::BNE(_)
            | Opcode::BEQ(_) => {}
        }
    }

    pub fn require_arg1(&self) -> bool {
        match self {
            Opcode::LDA(a)
            | Opcode::STA(a)
            | Opcode::AND(a)
            | Opcode::ADC(a)
            | Opcode::SBC(a)
            | Opcode::ASL(a)
            | Opcode::BIT(a)
            | Opcode::CMP(a)
            | Opcode::CPX(a)
            | Opcode::CPY(a)
            | Opcode::DEC(a)
            | Opcode::EOR(a)
            | Opcode::INC(a)
            | Opcode::JMP(a)
            | Opcode::JSR(a)
            | Opcode::LDX(a)
            | Opcode::LDY(a)
            | Opcode::LSR(a)
            | Opcode::ORA(a)
            | Opcode::ROL(a)
            | Opcode::ROR(a)
            | Opcode::STX(a)
            | Opcode::STY(a) => a.require_arg1(),
            Opcode::BPL(_)
            | Opcode::BMI(_)
            | Opcode::BVC(_)
            | Opcode::BVS(_)
            | Opcode::BCC(_)
            | Opcode::BCS(_)
            | Opcode::BNE(_)
            | Opcode::BEQ(_) => true,
            Opcode::NOP
            | Opcode::CLC
            | Opcode::SEC
            | Opcode::CLI
            | Opcode::SEI
            | Opcode::INY
            | Opcode::INX
            | Opcode::DEY
            | Opcode::DEX
            | Opcode::TAY
            | Opcode::TYA
            | Opcode::TAX
            | Opcode::TXA
            | Opcode::TXS
            | Opcode::TSX
            | Opcode::PHA
            | Opcode::PLA
            | Opcode::PHP
            | Opcode::PLP
            | Opcode::BRK
            | Opcode::CLV
            | Opcode::RTI
            | Opcode::RTS => false,
        }
    }
    pub fn require_arg2(&self) -> bool {
        match self {
            Opcode::LDA(a)
            | Opcode::STA(a)
            | Opcode::AND(a)
            | Opcode::ADC(a)
            | Opcode::SBC(a)
            | Opcode::ASL(a)
            | Opcode::BIT(a)
            | Opcode::CMP(a)
            | Opcode::CPX(a)
            | Opcode::CPY(a)
            | Opcode::DEC(a)
            | Opcode::EOR(a)
            | Opcode::INC(a)
            | Opcode::JMP(a)
            | Opcode::JSR(a)
            | Opcode::LDX(a)
            | Opcode::LDY(a)
            | Opcode::LSR(a)
            | Opcode::ORA(a)
            | Opcode::ROL(a)
            | Opcode::ROR(a)
            | Opcode::STX(a)
            | Opcode::STY(a) => a.require_arg2(),
            Opcode::NOP
            | Opcode::CLC
            | Opcode::SEC
            | Opcode::CLI
            | Opcode::SEI
            | Opcode::INY
            | Opcode::INX
            | Opcode::DEY
            | Opcode::DEX
            | Opcode::TAY
            | Opcode::TYA
            | Opcode::TAX
            | Opcode::TXA
            | Opcode::TXS
            | Opcode::TSX
            | Opcode::PHA
            | Opcode::PLA
            | Opcode::PHP
            | Opcode::PLP
            | Opcode::BRK
            | Opcode::CLV
            | Opcode::RTI
            | Opcode::RTS
            | Opcode::BPL(_)
            | Opcode::BMI(_)
            | Opcode::BVC(_)
            | Opcode::BVS(_)
            | Opcode::BCC(_)
            | Opcode::BCS(_)
            | Opcode::BNE(_)
            | Opcode::BEQ(_) => false,
        }
    }
    pub fn need_compute(&self) -> bool {
        match self {
            Opcode::LDA(a)
            | Opcode::STA(a)
            | Opcode::AND(a)
            | Opcode::ADC(a)
            | Opcode::SBC(a)
            | Opcode::ASL(a)
            | Opcode::BIT(a)
            | Opcode::CMP(a)
            | Opcode::CPX(a)
            | Opcode::CPY(a)
            | Opcode::DEC(a)
            | Opcode::EOR(a)
            | Opcode::INC(a)
            | Opcode::JMP(a)
            | Opcode::JSR(a)
            | Opcode::LDX(a)
            | Opcode::LDY(a)
            | Opcode::LSR(a)
            | Opcode::ORA(a)
            | Opcode::ROL(a)
            | Opcode::ROR(a)
            | Opcode::STX(a)
            | Opcode::STY(a) => a.need_compute(),
            Opcode::NOP
            | Opcode::CLC
            | Opcode::SEC
            | Opcode::CLI
            | Opcode::SEI
            | Opcode::INY
            | Opcode::INX
            | Opcode::DEY
            | Opcode::DEX
            | Opcode::TAY
            | Opcode::TYA
            | Opcode::TAX
            | Opcode::TXA
            | Opcode::TXS
            | Opcode::TSX
            | Opcode::PHA
            | Opcode::PLA
            | Opcode::PHP
            | Opcode::PLP
            | Opcode::BRK
            | Opcode::CLV
            | Opcode::RTI
            | Opcode::RTS
            | Opcode::BPL(_)
            | Opcode::BMI(_)
            | Opcode::BVC(_)
            | Opcode::BVS(_)
            | Opcode::BCC(_)
            | Opcode::BCS(_)
            | Opcode::BNE(_)
            | Opcode::BEQ(_) => false,
        }
    }
    pub fn compute(&mut self, cpu: &mut Nes6502, step: usize) {
        match self {
            Opcode::LDA(a)
            | Opcode::STA(a)
            | Opcode::AND(a)
            | Opcode::ADC(a)
            | Opcode::SBC(a)
            | Opcode::ASL(a)
            | Opcode::BIT(a)
            | Opcode::CMP(a)
            | Opcode::CPX(a)
            | Opcode::CPY(a)
            | Opcode::DEC(a)
            | Opcode::EOR(a)
            | Opcode::INC(a)
            | Opcode::JMP(a)
            | Opcode::JSR(a)
            | Opcode::LDX(a)
            | Opcode::LDY(a)
            | Opcode::LSR(a)
            | Opcode::ORA(a)
            | Opcode::ROL(a)
            | Opcode::ROR(a)
            | Opcode::STX(a)
            | Opcode::STY(a) => *a = a.compute(cpu, step),
            Opcode::NOP
            | Opcode::CLC
            | Opcode::SEC
            | Opcode::CLI
            | Opcode::SEI
            | Opcode::INY
            | Opcode::INX
            | Opcode::DEY
            | Opcode::DEX
            | Opcode::TAY
            | Opcode::TYA
            | Opcode::TAX
            | Opcode::TXA
            | Opcode::TXS
            | Opcode::TSX
            | Opcode::PHA
            | Opcode::PLA
            | Opcode::PHP
            | Opcode::PLP
            | Opcode::BRK
            | Opcode::CLV
            | Opcode::RTI
            | Opcode::RTS
            | Opcode::BPL(_)
            | Opcode::BMI(_)
            | Opcode::BVC(_)
            | Opcode::BVS(_)
            | Opcode::BCC(_)
            | Opcode::BCS(_)
            | Opcode::BNE(_)
            | Opcode::BEQ(_) => {}
        }
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x61 => Opcode::ADC(AddressingMode::IndexedIndirect(0)),
            0x65 => Opcode::ADC(AddressingMode::ZeroPage(0)),
            0x69 => Opcode::ADC(AddressingMode::Immediate(0)),
            0x6D => Opcode::ADC(AddressingMode::Absolute(0)),
            0x71 => Opcode::ADC(AddressingMode::IndirectIndexed(0)),
            0x75 => Opcode::ADC(AddressingMode::ZeroPageIndexedX(0)),
            0x79 => Opcode::ADC(AddressingMode::AbsoluteIndexedY(0)),
            0x7D => Opcode::ADC(AddressingMode::AbsoluteIndexedX(0)),

            0x21 => Opcode::AND(AddressingMode::IndexedIndirect(0)),
            0x25 => Opcode::AND(AddressingMode::ZeroPage(0)),
            0x29 => Opcode::AND(AddressingMode::Immediate(0)),
            0x2D => Opcode::AND(AddressingMode::Absolute(0)),
            0x31 => Opcode::AND(AddressingMode::IndirectIndexed(0)),
            0x35 => Opcode::AND(AddressingMode::ZeroPageIndexedX(0)),
            0x39 => Opcode::AND(AddressingMode::AbsoluteIndexedY(0)),
            0x3D => Opcode::AND(AddressingMode::AbsoluteIndexedX(0)),

            0x06 => Opcode::ASL(AddressingMode::ZeroPage(0)),
            0x0A => Opcode::ASL(AddressingMode::Implicit),
            0x0E => Opcode::ASL(AddressingMode::ZeroPageIndexedX(0)),
            0x16 => Opcode::ASL(AddressingMode::Absolute(0)),
            0x1E => Opcode::ASL(AddressingMode::AbsoluteIndexedX(0)),

            0x24 => Opcode::BIT(AddressingMode::ZeroPage(0)),
            0x2C => Opcode::BIT(AddressingMode::Absolute(0)),

            0x10 => Opcode::BPL(0),
            0x30 => Opcode::BMI(0),
            0x50 => Opcode::BVC(0),
            0x70 => Opcode::BVS(0),
            0x90 => Opcode::BCC(0),
            0xB0 => Opcode::BCS(0),
            0xD0 => Opcode::BNE(0),
            0xF0 => Opcode::BEQ(0),

            0x00 => Opcode::BRK,

            0xC1 => Opcode::CMP(AddressingMode::IndexedIndirect(0)),
            0xC5 => Opcode::CMP(AddressingMode::ZeroPage(0)),
            0xC9 => Opcode::CMP(AddressingMode::Immediate(0)),
            0xCD => Opcode::CMP(AddressingMode::Absolute(0)),
            0xD1 => Opcode::CMP(AddressingMode::IndirectIndexed(0)),
            0xD5 => Opcode::CMP(AddressingMode::ZeroPageIndexedX(0)),
            0xD9 => Opcode::CMP(AddressingMode::AbsoluteIndexedY(0)),
            0xDD => Opcode::CMP(AddressingMode::AbsoluteIndexedX(0)),

            0xE0 => Opcode::CPX(AddressingMode::Immediate(0)),
            0xE4 => Opcode::CPX(AddressingMode::ZeroPage(0)),
            0xEC => Opcode::CPX(AddressingMode::Absolute(0)),

            0xC0 => Opcode::CPY(AddressingMode::Immediate(0)),
            0xC4 => Opcode::CPY(AddressingMode::ZeroPage(0)),
            0xCC => Opcode::CPY(AddressingMode::Absolute(0)),

            0xC6 => Opcode::DEC(AddressingMode::ZeroPage(0)),
            0xCE => Opcode::DEC(AddressingMode::Absolute(0)),
            0xD6 => Opcode::DEC(AddressingMode::ZeroPageIndexedX(0)),
            0xDE => Opcode::DEC(AddressingMode::AbsoluteIndexedX(0)),

            0x41 => Opcode::EOR(AddressingMode::IndexedIndirect(0)),
            0x45 => Opcode::EOR(AddressingMode::ZeroPage(0)),
            0x49 => Opcode::EOR(AddressingMode::Immediate(0)),
            0x4D => Opcode::EOR(AddressingMode::Absolute(0)),
            0x51 => Opcode::EOR(AddressingMode::IndirectIndexed(0)),
            0x55 => Opcode::EOR(AddressingMode::ZeroPageIndexedX(0)),
            0x59 => Opcode::EOR(AddressingMode::AbsoluteIndexedY(0)),
            0x5D => Opcode::EOR(AddressingMode::AbsoluteIndexedX(0)),

            0x18 => Opcode::CLC,
            0x38 => Opcode::SEC,
            0x58 => Opcode::CLI,
            0x78 => Opcode::SEI,
            0xB8 => Opcode::CLV,
            // 0xD8 => Opcode::CLD,
            // 0xF8 => Opcode::SED,
            0xE6 => Opcode::INC(AddressingMode::ZeroPage(0)),
            0xEE => Opcode::INC(AddressingMode::Absolute(0)),
            0xF6 => Opcode::INC(AddressingMode::ZeroPageIndexedX(0)),
            0xFE => Opcode::INC(AddressingMode::AbsoluteIndexedX(0)),

            0x4C => Opcode::JMP(AddressingMode::Absolute(0)),
            0x6C => Opcode::JMP(AddressingMode::Indirect(0)),

            0x20 => Opcode::JSR(AddressingMode::Absolute(0)),

            0xA1 => Opcode::LDA(AddressingMode::IndexedIndirect(0)),
            0xA5 => Opcode::LDA(AddressingMode::ZeroPage(0)),
            0xA9 => Opcode::LDA(AddressingMode::Immediate(0)),
            0xAD => Opcode::LDA(AddressingMode::Absolute(0)),
            0xB1 => Opcode::LDA(AddressingMode::IndirectIndexed(0)),
            0xB5 => Opcode::LDA(AddressingMode::ZeroPageIndexedX(0)),
            0xB9 => Opcode::LDA(AddressingMode::AbsoluteIndexedY(0)),
            0xBD => Opcode::LDA(AddressingMode::AbsoluteIndexedX(0)),

            0xA2 => Opcode::LDX(AddressingMode::Immediate(0)),
            0xA6 => Opcode::LDX(AddressingMode::ZeroPage(0)),
            0xAE => Opcode::LDX(AddressingMode::Absolute(0)),
            0xB6 => Opcode::LDX(AddressingMode::ZeroPageIndexedY(0)),
            0xBE => Opcode::LDX(AddressingMode::AbsoluteIndexedY(0)),

            0xA0 => Opcode::LDY(AddressingMode::Immediate(0)),
            0xA4 => Opcode::LDY(AddressingMode::ZeroPage(0)),
            0xAC => Opcode::LDY(AddressingMode::Absolute(0)),
            0xB4 => Opcode::LDY(AddressingMode::ZeroPageIndexedY(0)),
            0xBC => Opcode::LDY(AddressingMode::AbsoluteIndexedY(0)),

            0x4A => Opcode::LSR(AddressingMode::Implicit),
            0x46 => Opcode::LSR(AddressingMode::ZeroPage(0)),
            0x4E => Opcode::LSR(AddressingMode::Absolute(0)),
            0x56 => Opcode::LSR(AddressingMode::ZeroPageIndexedX(0)),
            0x5E => Opcode::LSR(AddressingMode::AbsoluteIndexedX(0)),

            0xEA => Opcode::NOP,

            0x01 => Opcode::ORA(AddressingMode::IndexedIndirect(0)),
            0x05 => Opcode::ORA(AddressingMode::ZeroPage(0)),
            0x09 => Opcode::ORA(AddressingMode::Immediate(0)),
            0x0D => Opcode::ORA(AddressingMode::Absolute(0)),
            0x11 => Opcode::ORA(AddressingMode::IndirectIndexed(0)),
            0x15 => Opcode::ORA(AddressingMode::ZeroPageIndexedX(0)),
            0x19 => Opcode::ORA(AddressingMode::AbsoluteIndexedY(0)),
            0x1D => Opcode::ORA(AddressingMode::AbsoluteIndexedX(0)),

            0xAA => Opcode::TAX,
            0x8A => Opcode::TXA,
            0xCA => Opcode::DEX,
            0xE8 => Opcode::INX,
            0xA8 => Opcode::TAY,
            0x98 => Opcode::TYA,
            0x88 => Opcode::DEY,
            0xC8 => Opcode::INY,

            0x2A => Opcode::ROL(AddressingMode::Implicit),
            0x26 => Opcode::ROL(AddressingMode::ZeroPage(0)),
            0x2E => Opcode::ROL(AddressingMode::Absolute(0)),
            0x36 => Opcode::ROL(AddressingMode::ZeroPageIndexedX(0)),
            0x3E => Opcode::ROL(AddressingMode::AbsoluteIndexedX(0)),

            0x6A => Opcode::ROR(AddressingMode::Implicit),
            0x66 => Opcode::ROR(AddressingMode::ZeroPage(0)),
            0x6E => Opcode::ROR(AddressingMode::Absolute(0)),
            0x76 => Opcode::ROR(AddressingMode::ZeroPageIndexedX(0)),
            0x7E => Opcode::ROR(AddressingMode::AbsoluteIndexedX(0)),

            0x40 => Opcode::RTI,
            0x60 => Opcode::RTS,

            0xE1 => Opcode::SBC(AddressingMode::IndexedIndirect(0)),
            0xE5 => Opcode::SBC(AddressingMode::ZeroPage(0)),
            0xE9 => Opcode::SBC(AddressingMode::Immediate(0)),
            0xED => Opcode::SBC(AddressingMode::Absolute(0)),
            0xF1 => Opcode::SBC(AddressingMode::IndirectIndexed(0)),
            0xF5 => Opcode::SBC(AddressingMode::ZeroPageIndexedX(0)),
            0xF9 => Opcode::SBC(AddressingMode::AbsoluteIndexedY(0)),
            0xFD => Opcode::SBC(AddressingMode::AbsoluteIndexedX(0)),

            0x81 => Opcode::STA(AddressingMode::IndexedIndirect(0)),
            0x85 => Opcode::STA(AddressingMode::ZeroPage(0)),
            0x8D => Opcode::STA(AddressingMode::Absolute(0)),
            0x91 => Opcode::STA(AddressingMode::IndirectIndexed(0)),
            0x95 => Opcode::STA(AddressingMode::ZeroPageIndexedX(0)),
            0x99 => Opcode::STA(AddressingMode::AbsoluteIndexedY(0)),
            0x9D => Opcode::STA(AddressingMode::AbsoluteIndexedX(0)),

            0x9A => Opcode::TXS,
            0xBA => Opcode::TSX,
            0x48 => Opcode::PHA,
            0x68 => Opcode::PLA,
            0x08 => Opcode::PHP,
            0x28 => Opcode::PLP,

            0x86 => Opcode::STX(AddressingMode::ZeroPage(0)),
            0x8E => Opcode::STX(AddressingMode::Absolute(0)),
            0x96 => Opcode::STX(AddressingMode::ZeroPageIndexedY(0)),

            0x84 => Opcode::STY(AddressingMode::ZeroPage(0)),
            0x8C => Opcode::STY(AddressingMode::Absolute(0)),
            0x94 => Opcode::STY(AddressingMode::ZeroPageIndexedY(0)),

            _ => Opcode::NOP,
        }
    }
}

fn opcode_with_u16(opcode: u8, arg: u16) -> Vec<u8> {
    vec![opcode, (arg & 0xFF) as u8, (arg >> 8) as u8]
}

#[derive(Debug)]
pub enum ParseError {
    InvalidOpcode(String),
    InvalidAddressMode(String),
}

impl TryFrom<Opcode> for Vec<u8> {
    type Error = ParseError;

    fn try_from(value: Opcode) -> Result<Self, Self::Error> {
        match value {
            Opcode::ADC(AddressingMode::IndexedIndirect(a)) => Ok(vec![0x61, a]),
            Opcode::ADC(AddressingMode::ZeroPage(a)) => Ok(vec![0x65, a]),
            Opcode::ADC(AddressingMode::Immediate(a)) => Ok(vec![0x69, a]),
            Opcode::ADC(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x6D, a)),
            Opcode::ADC(AddressingMode::IndirectIndexed(a)) => Ok(vec![0x71, a]),
            Opcode::ADC(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0x75, a]),
            Opcode::ADC(AddressingMode::AbsoluteIndexedY(a)) => Ok(opcode_with_u16(0x79, a)),
            Opcode::ADC(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0x7D, a)),
            Opcode::ADC(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for ADC"
            ))),

            Opcode::AND(AddressingMode::IndexedIndirect(a)) => Ok(vec![0x21, a]),
            Opcode::AND(AddressingMode::ZeroPage(a)) => Ok(vec![0x25, a]),
            Opcode::AND(AddressingMode::Immediate(a)) => Ok(vec![0x29, a]),
            Opcode::AND(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x2D, a)),
            Opcode::AND(AddressingMode::IndirectIndexed(a)) => Ok(vec![0x31, a]),
            Opcode::AND(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0x35, a]),
            Opcode::AND(AddressingMode::AbsoluteIndexedY(a)) => Ok(opcode_with_u16(0x39, a)),
            Opcode::AND(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0x3D, a)),
            Opcode::AND(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for AND"
            ))),

            Opcode::ASL(AddressingMode::ZeroPage(a)) => Ok(vec![0x06, a]),
            Opcode::ASL(AddressingMode::Implicit) => Ok(vec![0x0A]),
            Opcode::ASL(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0x0E, a]),
            Opcode::ASL(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x16, a)),
            Opcode::ASL(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0x1E, a)),
            Opcode::ASL(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for ASL"
            ))),

            Opcode::BIT(AddressingMode::ZeroPage(a)) => Ok(vec![0x24, a]),
            Opcode::BIT(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x2C, a)),
            Opcode::BIT(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for BIT"
            ))),

            Opcode::BPL(a) => Ok(vec![0x10, a as u8]),
            Opcode::BMI(a) => Ok(vec![0x30, a as u8]),
            Opcode::BVC(a) => Ok(vec![0x50, a as u8]),
            Opcode::BVS(a) => Ok(vec![0x70, a as u8]),
            Opcode::BCC(a) => Ok(vec![0x90, a as u8]),
            Opcode::BCS(a) => Ok(vec![0xB0, a as u8]),
            Opcode::BNE(a) => Ok(vec![0xD0, a as u8]),
            Opcode::BEQ(a) => Ok(vec![0xF0, a as u8]),

            Opcode::BRK => Ok(vec![0x00]),

            Opcode::CMP(AddressingMode::IndexedIndirect(a)) => Ok(vec![0xC1, a]),
            Opcode::CMP(AddressingMode::ZeroPage(a)) => Ok(vec![0xC5, a]),
            Opcode::CMP(AddressingMode::Immediate(a)) => Ok(vec![0xC9, a]),
            Opcode::CMP(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0xCD, a)),
            Opcode::CMP(AddressingMode::IndirectIndexed(a)) => Ok(vec![0xD1, a]),
            Opcode::CMP(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0xD5, a]),
            Opcode::CMP(AddressingMode::AbsoluteIndexedY(a)) => Ok(opcode_with_u16(0xD9, a)),
            Opcode::CMP(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0xDD, a)),
            Opcode::CMP(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for CMP"
            ))),

            Opcode::CPX(AddressingMode::Immediate(a)) => Ok(vec![0xE0, a]),
            Opcode::CPX(AddressingMode::ZeroPage(a)) => Ok(vec![0xE4, a]),
            Opcode::CPX(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0xEC, a)),
            Opcode::CPX(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for CPX"
            ))),

            Opcode::CPY(AddressingMode::Immediate(a)) => Ok(vec![0xC0, a]),
            Opcode::CPY(AddressingMode::ZeroPage(a)) => Ok(vec![0xC4, a]),
            Opcode::CPY(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0xCC, a)),
            Opcode::CPY(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for CPY"
            ))),

            Opcode::DEC(AddressingMode::ZeroPage(a)) => Ok(vec![0xC6, a]),
            Opcode::DEC(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0xCE, a)),
            Opcode::DEC(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0xD6, a]),
            Opcode::DEC(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0xDE, a)),
            Opcode::DEC(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for DEC"
            ))),

            Opcode::EOR(AddressingMode::IndexedIndirect(a)) => Ok(vec![0x41, a]),
            Opcode::EOR(AddressingMode::ZeroPage(a)) => Ok(vec![0x45, a]),
            Opcode::EOR(AddressingMode::Immediate(a)) => Ok(vec![0x49, a]),
            Opcode::EOR(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x4D, a)),
            Opcode::EOR(AddressingMode::IndirectIndexed(a)) => Ok(vec![0x51, a]),
            Opcode::EOR(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0x55, a]),
            Opcode::EOR(AddressingMode::AbsoluteIndexedY(a)) => Ok(opcode_with_u16(0x59, a)),
            Opcode::EOR(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0x5D, a)),
            Opcode::EOR(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for EOR"
            ))),

            Opcode::CLC => Ok(vec![0x18]),
            Opcode::SEC => Ok(vec![0x38]),
            Opcode::CLI => Ok(vec![0x58]),
            Opcode::SEI => Ok(vec![0x78]),
            Opcode::CLV => Ok(vec![0xB8]),

            Opcode::INC(AddressingMode::ZeroPage(a)) => Ok(vec![0xE6, a]),
            Opcode::INC(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0xEE, a)),
            Opcode::INC(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0xF6, a]),
            Opcode::INC(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0xFE, a)),
            Opcode::INC(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for INC"
            ))),

            Opcode::JMP(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x4C, a)),
            Opcode::JMP(AddressingMode::Indirect(a)) => Ok(opcode_with_u16(0x6C, a)),
            Opcode::JMP(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for JMP"
            ))),

            Opcode::JSR(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x20, a)),
            Opcode::JSR(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for JSR"
            ))),

            Opcode::LDA(AddressingMode::IndexedIndirect(a)) => Ok(vec![0xA1, a]),
            Opcode::LDA(AddressingMode::ZeroPage(a)) => Ok(vec![0xA5, a]),
            Opcode::LDA(AddressingMode::Immediate(a)) => Ok(vec![0xA9, a]),
            Opcode::LDA(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0xAD, a)),
            Opcode::LDA(AddressingMode::IndirectIndexed(a)) => Ok(vec![0xB1, a]),
            Opcode::LDA(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0xB5, a]),
            Opcode::LDA(AddressingMode::AbsoluteIndexedY(a)) => Ok(opcode_with_u16(0xB9, a)),
            Opcode::LDA(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0xBD, a)),
            Opcode::LDA(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for LDA"
            ))),

            Opcode::LDX(AddressingMode::Immediate(a)) => Ok(vec![0xA2, a]),
            Opcode::LDX(AddressingMode::ZeroPage(a)) => Ok(vec![0xA6, a]),
            Opcode::LDX(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0xAE, a)),
            Opcode::LDX(AddressingMode::ZeroPageIndexedY(a)) => Ok(vec![0xB6, a]),
            Opcode::LDX(AddressingMode::AbsoluteIndexedY(a)) => Ok(opcode_with_u16(0xBE, a)),
            Opcode::LDX(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for LDX"
            ))),

            Opcode::LDY(AddressingMode::Immediate(a)) => Ok(vec![0xA0, a]),
            Opcode::LDY(AddressingMode::ZeroPage(a)) => Ok(vec![0xA4, a]),
            Opcode::LDY(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0xAC, a)),
            Opcode::LDY(AddressingMode::ZeroPageIndexedY(a)) => Ok(vec![0xB4, a]),
            Opcode::LDY(AddressingMode::AbsoluteIndexedY(a)) => Ok(opcode_with_u16(0xBC, a)),
            Opcode::LDY(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for LDY"
            ))),

            Opcode::LSR(AddressingMode::Implicit) => Ok(vec![0x4A]),
            Opcode::LSR(AddressingMode::ZeroPage(a)) => Ok(vec![0x46, a]),
            Opcode::LSR(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x4E, a)),
            Opcode::LSR(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0x56, a]),
            Opcode::LSR(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0x5E, a)),
            Opcode::LSR(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for LSR"
            ))),

            Opcode::NOP => Ok(vec![0xEA]),

            Opcode::ORA(AddressingMode::IndexedIndirect(a)) => Ok(vec![0x01, a]),
            Opcode::ORA(AddressingMode::ZeroPage(a)) => Ok(vec![0x05, a]),
            Opcode::ORA(AddressingMode::Immediate(a)) => Ok(vec![0x09, a]),
            Opcode::ORA(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x0D, a)),
            Opcode::ORA(AddressingMode::IndirectIndexed(a)) => Ok(vec![0x11, a]),
            Opcode::ORA(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0x15, a]),
            Opcode::ORA(AddressingMode::AbsoluteIndexedY(a)) => Ok(opcode_with_u16(0x19, a)),
            Opcode::ORA(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0x1D, a)),
            Opcode::ORA(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for ORA"
            ))),

            Opcode::TAX => Ok(vec![0xAA]),
            Opcode::TXA => Ok(vec![0x8A]),
            Opcode::DEX => Ok(vec![0xCA]),
            Opcode::INX => Ok(vec![0xE8]),
            Opcode::TAY => Ok(vec![0xA8]),
            Opcode::TYA => Ok(vec![0x98]),
            Opcode::DEY => Ok(vec![0x88]),
            Opcode::INY => Ok(vec![0xC8]),

            Opcode::ROL(AddressingMode::Implicit) => Ok(vec![0x2A]),
            Opcode::ROL(AddressingMode::ZeroPage(a)) => Ok(vec![0x26, a]),
            Opcode::ROL(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x2E, a)),
            Opcode::ROL(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0x36, a]),
            Opcode::ROL(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0x3E, a)),
            Opcode::ROL(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for ROL"
            ))),

            Opcode::ROR(AddressingMode::Implicit) => Ok(vec![0x6A]),
            Opcode::ROR(AddressingMode::ZeroPage(a)) => Ok(vec![0x66, a]),
            Opcode::ROR(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x6E, a)),
            Opcode::ROR(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0x76, a]),
            Opcode::ROR(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0x7E, a)),
            Opcode::ROR(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for ROR"
            ))),

            Opcode::RTI => Ok(vec![0x40]),
            Opcode::RTS => Ok(vec![0x60]),

            Opcode::SBC(AddressingMode::IndexedIndirect(a)) => Ok(vec![0xE1, a]),
            Opcode::SBC(AddressingMode::ZeroPage(a)) => Ok(vec![0xE5, a]),
            Opcode::SBC(AddressingMode::Immediate(a)) => Ok(vec![0xE9, a]),
            Opcode::SBC(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0xED, a)),
            Opcode::SBC(AddressingMode::IndirectIndexed(a)) => Ok(vec![0xF1, a]),
            Opcode::SBC(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0xF5, a]),
            Opcode::SBC(AddressingMode::AbsoluteIndexedY(a)) => Ok(opcode_with_u16(0xF9, a)),
            Opcode::SBC(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0xFD, a)),
            Opcode::SBC(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for SBC"
            ))),

            Opcode::STA(AddressingMode::IndexedIndirect(a)) => Ok(vec![0x81, a]),
            Opcode::STA(AddressingMode::ZeroPage(a)) => Ok(vec![0x85, a]),
            Opcode::STA(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x8D, a)),
            Opcode::STA(AddressingMode::IndirectIndexed(a)) => Ok(vec![0x91, a]),
            Opcode::STA(AddressingMode::ZeroPageIndexedX(a)) => Ok(vec![0x95, a]),
            Opcode::STA(AddressingMode::AbsoluteIndexedY(a)) => Ok(opcode_with_u16(0x99, a)),
            Opcode::STA(AddressingMode::AbsoluteIndexedX(a)) => Ok(opcode_with_u16(0x9D, a)),
            Opcode::STA(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for STA"
            ))),

            Opcode::TXS => Ok(vec![0x9A]),
            Opcode::TSX => Ok(vec![0xBA]),
            Opcode::PHA => Ok(vec![0x48]),
            Opcode::PLA => Ok(vec![0x68]),
            Opcode::PHP => Ok(vec![0x08]),
            Opcode::PLP => Ok(vec![0x28]),

            Opcode::STX(AddressingMode::ZeroPage(a)) => Ok(vec![0x86, a]),
            Opcode::STX(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x8E, a)),
            Opcode::STX(AddressingMode::ZeroPageIndexedY(a)) => Ok(vec![0x96, a]),
            Opcode::STX(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for STX"
            ))),

            Opcode::STY(AddressingMode::ZeroPage(a)) => Ok(vec![0x84, a]),
            Opcode::STY(AddressingMode::Absolute(a)) => Ok(opcode_with_u16(0x8C, a)),
            Opcode::STY(AddressingMode::ZeroPageIndexedY(a)) => Ok(vec![0x94, a]),
            Opcode::STY(a) => Err(ParseError::InvalidAddressMode(format!(
                "Invalid Addressing mode {a:?} for STY"
            ))),
        }
    }
}
