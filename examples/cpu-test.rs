use virt_ic::*;
use virt_ic::chip::gates::*;
use virt_ic::chip::generators::*;
use virt_ic::chip::memory::*;
use virt_ic::chip::cpu::*;
use virt_ic::chip::clocks::*;
use std::time::Duration;

#[allow(dead_code)]
const HLT: u8 = 0x00;
#[allow(dead_code)]
const INA: u8 = 0x01;
#[allow(dead_code)]
const DEA: u8 = 0x02;
#[allow(dead_code)]
const INL: u8 = 0x03;
#[allow(dead_code)]
const DEL: u8 = 0x04;
#[allow(dead_code)]
const CLC: u8 = 0x05;
#[allow(dead_code)]
const ADB: u8 = 0x06;
#[allow(dead_code)]
const ADC: u8 = 0x07;
#[allow(dead_code)]
const TAB: u8 = 0x08;
#[allow(dead_code)]
const TBA: u8 = 0x09;
#[allow(dead_code)]
const TAC: u8 = 0x0A;
#[allow(dead_code)]
const TCA: u8 = 0x0B;
#[allow(dead_code)]
const TAH: u8 = 0x0C;
#[allow(dead_code)]
const THA: u8 = 0x0D;
#[allow(dead_code)]
const TAL: u8 = 0x0E;
#[allow(dead_code)]
const TLA: u8 = 0x0F;
#[allow(dead_code)]
const PHA: u8 = 0x10;
#[allow(dead_code)]
const PLA: u8 = 0x11;
#[allow(dead_code)]
const PHL: u8 = 0x12;
#[allow(dead_code)]
const PLL: u8 = 0x13;
#[allow(dead_code)]
const CPB_ACC: u8 = 0x14;
#[allow(dead_code)]
const CPC_ACC: u8 = 0x15;
#[allow(dead_code)]
const SUB: u8 = 0x16;
#[allow(dead_code)]
const SUC: u8 = 0x17;
#[allow(dead_code)]
const SAL: u8 = 0x18;
#[allow(dead_code)]
const SAR: u8 = 0x19;
#[allow(dead_code)]
const INB: u8 = 0x1A;
#[allow(dead_code)]
const DEB: u8 = 0x1B;
#[allow(dead_code)]
const INC: u8 = 0x1C;
#[allow(dead_code)]
const DEC: u8 = 0x1D;
#[allow(dead_code)]
const JML: u8 = 0x20;
#[allow(dead_code)]
const JSL: u8 = 0x21;
#[allow(dead_code)]
const RTN: u8 = 0x22;
#[allow(dead_code)]
const STA_HL: u8 = 0x48;
#[allow(dead_code)]
const STB_HL: u8 = 0x49;
#[allow(dead_code)]
const STC_HL: u8 = 0x4A;
#[allow(dead_code)]
const LDB_HL: u8 = 0x4B;
#[allow(dead_code)]
const LDC_HL: u8 = 0x4C;
#[allow(dead_code)]
const LDA_HL: u8 = 0x4D;
#[allow(dead_code)]
const LDA_HLB: u8 = 0x4E;
#[allow(dead_code)]
const LDA_HLC: u8 = 0x4F;
#[allow(dead_code)]
const LDA_NB: u8 = 0x50;
#[allow(dead_code)]
const LDA_ZP: u8 = 0x51;
#[allow(dead_code)]
const LDA_H0P: u8 = 0x52;
#[allow(dead_code)]
const LDA_HLP: u8 = 0x53;
#[allow(dead_code)]
const LDB_NB: u8 = 0x54;
#[allow(dead_code)]
const LDB_ZP: u8 = 0x55;
#[allow(dead_code)]
const LDC_NB: u8 = 0x56;
#[allow(dead_code)]
const LDC_ZP: u8 = 0x57;
#[allow(dead_code)]
const STA_ZP: u8 = 0x58;
#[allow(dead_code)]
const STA_H0P: u8 = 0x59;
#[allow(dead_code)]
const STA_HLP: u8 = 0x5A;
#[allow(dead_code)]
const STB_ZP: u8 = 0x5B;
#[allow(dead_code)]
const STC_ZP: u8 = 0x5C;
#[allow(dead_code)]
const CMP: u8 = 0x60;
#[allow(dead_code)]
const CPB_NB: u8 = 0x61;
#[allow(dead_code)]
const CPC_NB: u8 = 0x62;
#[allow(dead_code)]
const JMP: u8 = 0xB0;
#[allow(dead_code)]
const JSR: u8 = 0xB1;
#[allow(dead_code)]
const BCF: u8 = 0xB2;
#[allow(dead_code)]
const BNF: u8 = 0xB3;
#[allow(dead_code)]
const BZF: u8 = 0xB4;
#[allow(dead_code)]
const LDA_ADR: u8 = 0xC0;
#[allow(dead_code)]
const STA_ADR: u8 = 0xC1;

fn main() {
    // basic CPU setup with the following mapping :
    // RAM = 0x000 to 0x0FF
    // ROM = 0xF00 to 0xFFF
    // Stack will be on bank 0x0
    let mut board = Board::new();
    let cpu = board.new_socket_with(Box::new(SimpleCPU::new()));
    let ram = board.new_socket_with(Box::new(Ram256B::new()));
    // rom chip with a simple factorial calculation program
    // pre-compiled to perform a factorial of 4
    let rom = board.new_socket_with(Box::new(Rom256B::from_data(
        [
            // init
            // 0x00
            JSR, 0x0F, 0x28,// JSR :zeroing ram
            // 0x03
            LDA_NB, 0x00,   // A = 0
            // 0x05
            TAH,TAL,        // HL = 0
            // 0x07
            LDA_NB, 0x01,   // A = 1
            // 0x09
            LDC_NB, 0x02,   // C = 2

            // loop
            // 0x0B
            STA_HL,         // [0xHL] = A
            // 0x0C
            INL,            // HL++
            // 0x0D
            TAB,            // B = A
            // 0x0E
            CPC_NB, 0x05,   // CMP C with 5
            // 0x10
            BZF, 0x0F, 0x27,// IF Z JMP :end
            // 0x13
            LDA_NB, 0x00,   // A = 0
            // 0x15
            JSR, 0x0F, 0x1C,// JSR :multiply routine : A = B*C
            // 0x18
            INC,            // C++
            // 0x19
            JMP, 0x0F, 0x0B,// JMP :loop

            // multiply routine A += B*C
            // 0x1C
            CPB_NB, 0x00,   // CMP B with 0
            // 0x1E
            BZF, 0x0F, 0x26,// IF Z JMP :multiply rtn
            // 0x21
            ADC,            // A += C
            // 0x22
            DEB,            // B--
            // 0x23
            JMP, 0x0F, 0x1C,// JMP :multiply routine
            // multiply rtn
            // 0x26
            RTN,            // Return
            // end
            // 0x27
            HLT,            // stop the processor
            // zeroing ram
            // 0x28
            LDA_NB, 0x00,
            // 0x2A
            STA_HL,         // [HL] = 0
            // 0x2B
            INL,            // HL++
            // 0x2C
            TLA,            // A = L
            // 0x2D
            CMP, 0x10,      // CMP A with 0x10
            // 0x2F
            BZF, 0x0F, 0x35,// IF Z JMP :zeroing rtn
            // 0x32
            JMP, 0x0F, 0x28,// JMP :zeroing ram
            // zeroing rtn
            // 0x35
            RTN,            // Return
            // padding
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            // IRQ address at 0xF00
            0x0F,0x00,
            // startup address at 0xF00
            0x0F,0x00,
            // stack bank at 0x000 to 0x0FF
            0x00
        ]
    )));

    let gen = board.new_socket_with(Box::new(Generator::new()));

    let clk = board.new_socket_with(Box::new(Clock100Hz::new()));

    let and = board.new_socket_with(Box::new(GateAnd::new()));
    let not = board.new_socket_with(Box::new(GateNot::new()));

    {
        // VCC
        let trc = board.new_trace();
        trc.borrow_mut().connect(gen.borrow_mut().get_pin(Generator::VCC).unwrap());
        trc.borrow_mut().connect(ram.borrow_mut().get_pin(Ram256B::VCC).unwrap());
        trc.borrow_mut().connect(rom.borrow_mut().get_pin(Rom256B::VCC).unwrap());
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::VCC).unwrap());
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::RESET).unwrap());
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::IRQ).unwrap());
        trc.borrow_mut().connect(clk.borrow_mut().get_pin(Clock100Hz::VCC).unwrap());
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::VCC).unwrap());
        trc.borrow_mut().connect(not.borrow_mut().get_pin(GateNot::VCC).unwrap());
    }
    {
        // GND
        let trc = board.new_trace();
        trc.borrow_mut().connect(gen.borrow_mut().get_pin(Generator::GND).unwrap());
        trc.borrow_mut().connect(ram.borrow_mut().get_pin(Ram256B::GND).unwrap());
        trc.borrow_mut().connect(rom.borrow_mut().get_pin(Rom256B::GND).unwrap());
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::GND).unwrap());
        trc.borrow_mut().connect(clk.borrow_mut().get_pin(Clock100Hz::GND).unwrap());
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::GND).unwrap());
        trc.borrow_mut().connect(not.borrow_mut().get_pin(GateNot::GND).unwrap());
    }
    {
        // CLK
        let trc = board.new_trace();
        trc.borrow_mut().connect(clk.borrow_mut().get_pin(Clock100Hz::CLK).unwrap());
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::CLOCK).unwrap());
    }
    {
        // AND
        // link A&B with C&D to make (A&B)&(C&D)
        // also link the result in a not gate
        let trc = board.new_trace();
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::A_AND_B).unwrap());
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::G).unwrap());
        
        let trc = board.new_trace();
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::C_AND_D).unwrap());
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::H).unwrap());
    }

    // CPU connections
    for i in 0..=6 {
        // A0 - A6
        let trc = board.new_trace();
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::A0+i).unwrap());
        trc.borrow_mut().connect(ram.borrow_mut().get_pin(Ram256B::A0+i).unwrap());
        trc.borrow_mut().connect(rom.borrow_mut().get_pin(Rom256B::A0+i).unwrap());
    }
    {
        // A7
        let trc = board.new_trace();
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::A7).unwrap());
        trc.borrow_mut().connect(ram.borrow_mut().get_pin(Ram256B::A7).unwrap());
        trc.borrow_mut().connect(rom.borrow_mut().get_pin(Rom256B::A7).unwrap());
    }
    {
        // CPU A8 - 12
        let trc = board.new_trace();
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::A8).unwrap());
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::A).unwrap());
        
        let trc = board.new_trace();
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::A9).unwrap());
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::B).unwrap());
        
        let trc = board.new_trace();
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::A10).unwrap());
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::C).unwrap());
        
        let trc = board.new_trace();
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::A11).unwrap());
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::D).unwrap());

        let trc = board.new_trace();
        trc.borrow_mut().connect(and.borrow_mut().get_pin(GateAnd::G_AND_H).unwrap());
        trc.borrow_mut().connect(not.borrow_mut().get_pin(GateNot::A).unwrap());
        trc.borrow_mut().connect(ram.borrow_mut().get_pin(Ram256B::CS).unwrap());

    }
    for i in 0..=7{
        // CPU IO0-7
        let trc = board.new_trace();
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::IO0+i).unwrap());
        trc.borrow_mut().connect(ram.borrow_mut().get_pin(Ram256B::IO0+i).unwrap());
        trc.borrow_mut().connect(rom.borrow_mut().get_pin(Rom256B::IO0+i).unwrap());
    }
    {
        // Ram and Rom CS, WE and OE

        let trc = board.new_trace();
        trc.borrow_mut().connect(not.borrow_mut().get_pin(GateNot::NOT_A).unwrap());
        trc.borrow_mut().connect(rom.borrow_mut().get_pin(Rom256B::CS).unwrap());

        let trc = board.new_trace();
        trc.borrow_mut().connect(cpu.borrow_mut().get_pin(SimpleCPU::RW).unwrap());
        trc.borrow_mut().connect(ram.borrow_mut().get_pin(Ram256B::WE).unwrap());
        trc.borrow_mut().connect(not.borrow_mut().get_pin(GateNot::B).unwrap());

        let trc = board.new_trace();
        trc.borrow_mut().connect(not.borrow_mut().get_pin(GateNot::NOT_B).unwrap());
        trc.borrow_mut().connect(ram.borrow_mut().get_pin(Ram256B::OE).unwrap());
        trc.borrow_mut().connect(rom.borrow_mut().get_pin(Rom256B::OE).unwrap());
    }
    board.run(Duration::from_millis(1));

    println!("ROM:\n{:?}", rom.borrow_mut().get_chip().as_ref().unwrap());
    println!("RAM before:\n{:?}", ram.borrow_mut().get_chip().as_ref().unwrap());

    board.run_during(Duration::from_secs(10), Duration::from_millis(1));

    println!("RAM after:\n{:?}", ram.borrow_mut().get_chip().as_ref().unwrap());
    println!("CPU:\n{:?}", cpu.borrow_mut().get_chip().as_ref().unwrap());
}