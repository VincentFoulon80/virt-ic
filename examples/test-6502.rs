use std::time::Duration;

use virt_ic::{
    board::{Board, Trace},
    chip::{
        clocks::Clock,
        cpu::{nes6502, Nes6502},
        gates::NotGate,
        generators::Generator,
        memories::{Ram256B, Rom256B},
        ChipBuilder, ChipSet,
    },
};

/// The main goal here is to test the CPU features within virt-ic simulation.
///
/// This example follows an extremely simple setup :
/// - One RAM chip of 256 Bytes is mapped and mirrored throughout $0000 -> $7FFF
/// - One ROM chip of 256 Bytes is mapped and mirrored throughout $8000 -> $FFFF
/// - One 50 Hz clock to provide a clock signal to the CPU
/// - One 6-NOT gate chip to do some glue logic between the CPU, ROM and RAM chips.
/// - One generator to power everything
///
/// The CPU will run a simple test program that is assembled using the Opcode enum.
///
/// This example will show the content of ROM, RAM then run the CPU for a certain amount of time,
/// and finally show the content of RAM after the simulation has ended.
fn main() {
    let mut board: Board<ChipSet> = Board::new();

    // assemble a 6502 program
    let mut prg = nes6502::Assembler::assemble(&[
        // first do some addition and substraction
        nes6502::Opcode::CLC,
        nes6502::Opcode::LDA(nes6502::AddressingMode::Immediate(0x5A)),
        nes6502::Opcode::ADC(nes6502::AddressingMode::Immediate(0xFF)),
        nes6502::Opcode::SEC,
        nes6502::Opcode::SBC(nes6502::AddressingMode::Immediate(0xFF)),
        // then setup a loop that'll fill the first 10 bytes of RAM with the content of the last byte in RAM
        nes6502::Opcode::LDX(nes6502::AddressingMode::Immediate(0x0A)),
        nes6502::Opcode::LDA(nes6502::AddressingMode::ZeroPage(0xFF)),
        nes6502::Opcode::STA(nes6502::AddressingMode::ZeroPageIndexedX(0x00)),
        nes6502::Opcode::DEX,
        nes6502::Opcode::BPL(-5),
        // infinite loop to halt the program
        nes6502::Opcode::BMI(-2),
    ])
    .unwrap();
    // resize assembled program and write 6502's reset vector
    prg.resize(256, 0);
    prg[0xFC] = 0x80;
    prg[0xFD] = 0x00;

    let rom = board.register_chip(Rom256B::build().set_data(prg.as_slice()).into());
    let ram = board.register_chip(Ram256B::build());

    let not = board.register_chip(NotGate::build());

    let vcc = board.register_chip(Generator::build().into());
    let clock = board.register_chip(Clock::build().with_frequency(50.0).into());

    let cpu = board.register_chip(Nes6502::build());

    board.register_trace(Trace::from(vec![
        (vcc, Generator::OUT),
        (clock, Clock::VCC),
        (cpu, Nes6502::VCC),
        (not, NotGate::VCC),
        (rom, Rom256B::VCC),
        (ram, Ram256B::VCC),
    ]));

    // connect address and data lines to ROM
    board.connect(cpu, Nes6502::A0, rom, Rom256B::A0);
    board.connect(cpu, Nes6502::A1, rom, Rom256B::A1);
    board.connect(cpu, Nes6502::A2, rom, Rom256B::A2);
    board.connect(cpu, Nes6502::A3, rom, Rom256B::A3);
    board.connect(cpu, Nes6502::A4, rom, Rom256B::A4);
    board.connect(cpu, Nes6502::A5, rom, Rom256B::A5);
    board.connect(cpu, Nes6502::A6, rom, Rom256B::A6);
    board.connect(cpu, Nes6502::A7, rom, Rom256B::A7);
    board.connect(cpu, Nes6502::D0, rom, Rom256B::IO0);
    board.connect(cpu, Nes6502::D1, rom, Rom256B::IO1);
    board.connect(cpu, Nes6502::D2, rom, Rom256B::IO2);
    board.connect(cpu, Nes6502::D3, rom, Rom256B::IO3);
    board.connect(cpu, Nes6502::D4, rom, Rom256B::IO4);
    board.connect(cpu, Nes6502::D5, rom, Rom256B::IO5);
    board.connect(cpu, Nes6502::D6, rom, Rom256B::IO6);
    board.connect(cpu, Nes6502::D7, rom, Rom256B::IO7);
    // connect NOT A15 to ROM's CS
    board.connect(cpu, Nes6502::A15, not, NotGate::A);
    board.connect(not, NotGate::NOT_A, rom, Rom256B::CS);

    // connect address and data lines to RAM
    board.connect(cpu, Nes6502::A0, ram, Ram256B::A0);
    board.connect(cpu, Nes6502::A1, ram, Ram256B::A1);
    board.connect(cpu, Nes6502::A2, ram, Ram256B::A2);
    board.connect(cpu, Nes6502::A3, ram, Ram256B::A3);
    board.connect(cpu, Nes6502::A4, ram, Ram256B::A4);
    board.connect(cpu, Nes6502::A5, ram, Ram256B::A5);
    board.connect(cpu, Nes6502::A6, ram, Ram256B::A6);
    board.connect(cpu, Nes6502::A7, ram, Ram256B::A7);
    board.connect(cpu, Nes6502::D0, ram, Ram256B::IO0);
    board.connect(cpu, Nes6502::D1, ram, Ram256B::IO1);
    board.connect(cpu, Nes6502::D2, ram, Ram256B::IO2);
    board.connect(cpu, Nes6502::D3, ram, Ram256B::IO3);
    board.connect(cpu, Nes6502::D4, ram, Ram256B::IO4);
    board.connect(cpu, Nes6502::D5, ram, Ram256B::IO5);
    board.connect(cpu, Nes6502::D6, ram, Ram256B::IO6);
    board.connect(cpu, Nes6502::D7, ram, Ram256B::IO7);
    // connect A15 to RAM's CS
    board.connect(cpu, Nes6502::A15, ram, Ram256B::CS);

    // connect NOT WR to ROM and RAM's OE, and to RAM's WE
    board.register_trace(Trace::from(vec![
        (cpu, Nes6502::RW),
        (ram, Ram256B::WE),
        (not, NotGate::B),
    ]));
    board.register_trace(Trace::from(vec![
        (not, NotGate::NOT_B),
        (rom, Rom256B::OE),
        (ram, Ram256B::OE),
    ]));

    board.connect(clock, Clock::CLK, cpu, Nes6502::CLK);

    board.run(Duration::from_millis(1));

    if let Some(ChipSet::Rom256B(rom)) = board.get_chip(&rom) {
        println!("ROM CONTENT");
        println!("{}", rom.to_string());
    }
    if let Some(ChipSet::Ram256B(ram)) = board.get_chip(&ram) {
        println!("RAM CONTENT");
        println!("{}", ram.to_string());
    }

    // run the simulation at 50Hz for 3.2 seconds
    for _ in 0..160 {
        board.run_realtime(Duration::from_millis(20));

        if let Some(ChipSet::Nes6502(cpu)) = board.get_chip(&cpu) {
            println!("{}", cpu.to_string());
        }
    }

    if let Some(ChipSet::Ram256B(ram)) = board.get_chip(&ram) {
        println!("RAM CONTENT");
        println!("{}", ram.to_string());
    }
}
