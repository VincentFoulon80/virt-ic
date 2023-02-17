use std::time::Duration;

use ron::ser::PrettyConfig;
use virt_ic::{
    board::{Board, Trace},
    chip::{gates::AndGate, generators::Generator, ChipBuilder, ChipType},
};

fn main() {
    let mut board: Board<ChipType> = Board::new();

    let and_gate = board.register_chip(AndGate::build());

    let vcc = board.register_chip(Generator::build().into());

    let mut trace = Trace::new();
    trace.connect(vcc, Generator::OUT);
    trace.connect(and_gate, AndGate::VCC);
    trace.connect(and_gate, AndGate::A);
    trace.connect(and_gate, AndGate::B);
    trace.connect(and_gate, AndGate::C);
    trace.connect(and_gate, AndGate::F);

    let trace = board.register_trace(trace);

    board.run(Duration::from_millis(10));

    let saved = ron::ser::to_string_pretty(&board, PrettyConfig::default()).unwrap();

    println!("{}", saved);

    let mut board2: Board<ChipType> = ron::de::from_str(&saved).unwrap();

    board2
        .get_trace_mut(&trace)
        .disconnect(and_gate, AndGate::A);

    board2.run(Duration::from_millis(10));

    dbg!(board2);
}
