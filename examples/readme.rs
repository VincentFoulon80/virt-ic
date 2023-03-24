use std::time::Duration;

use virt_ic::{
    board::{Board, Trace},
    chip::{gates::AndGate, generators::Generator, Chip, ChipBuilder, ChipSet},
};

fn main() {
    // create a new board
    let mut board: Board<ChipSet> = Board::new();
    // place an AND gate to the board
    let and_gate = board.register_chip(AndGate::build());
    // also place a generator
    let vcc = board.register_chip(Generator::build().into());
    let gnd = board.register_chip(Generator::build().with_state(virt_ic::State::Low).into());

    // Connect the AndGate's VCC, A and B pins with the Generator
    let mut trace = Trace::new();
    trace.connect(vcc, Generator::OUT);
    trace.connect(and_gate, AndGate::VCC);
    trace.connect(and_gate, AndGate::A);
    trace.connect(and_gate, AndGate::B);
    let trace_vcc = board.register_trace(trace);

    // Alternative way to connect chips via board, connect GND pins
    let trace_gnd = board.connect(gnd, Generator::OUT, and_gate, AndGate::GND);

    // simulate the board for 10ms
    board.run(Duration::from_millis(10));

    // check the results
    if let Some(chip) = board.get_chip(&and_gate) {
        println!(
            "A={:?}, \tB={:?}, \tA&B={:?}",
            chip.get_pin(AndGate::A).map(|p| p.state),
            chip.get_pin(AndGate::B).map(|p| p.state),
            chip.get_pin(AndGate::AB).map(|p| p.state)
        );
    }

    // disconnect AndGate's pin B from VCC and connect it instead to GND
    if let Some(t) = board.get_trace_mut(&trace_vcc) {
        t.disconnect(and_gate, AndGate::B)
    }
    if let Some(t) = board.get_trace_mut(&trace_gnd) {
        t.connect(and_gate, AndGate::B)
    }

    // simulate the board for another 10ms
    board.run(Duration::from_millis(10));

    // check the results
    if let Some(chip) = board.get_chip(&and_gate) {
        println!(
            "A={:?}, \tB={:?}, \tA&B={:?}",
            chip.get_pin(AndGate::A).map(|p| p.state),
            chip.get_pin(AndGate::B).map(|p| p.state),
            chip.get_pin(AndGate::AB).map(|p| p.state)
        );
    }
}
