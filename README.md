# Virtual Integrated Circuits

[Changelog](https://github.com/VincentFoulon80/virt-ic/releases)

This library is a Integrated Circuit Emulator backend that can simulate interactions between multiple chips.

Note that for now there is mostly Digital circuit emulation, Analog signals are kinda present but still need work.

You start by creating a Board, register Chips and Traces, and link Chip pins together to form a virtual circuit.
You can then run the circuit to emulate the chips and links between them.

This library is a Backend emulator, it means that there is no GUI to create boards.

# Note on 0.5.0 update

The entire library has been rewritten from scratch in order to ease the use of this crate, remove all those `Rc<RefCell>` that were degrading the readability of your code. Thus, virt-ic up before 0.5.0 is **completely incompatible** with newer versions.

# Features

- Build Boards with chips and traces between them
- Simulate the board for a certain duration with a certain step, it's also possible to run it in realtime !
- Save and load the board to backup your design or continue your simulation later

## Available Built-in Chips

- Generator
- Logic Gates (And, Or, Not, Nand, Nor)
- Button
- Clock
- Memory (RAM, ROM)
- Segment display
- CPU (a 6502, missing interrupts and decimal mode)

# Contributing

This project is open to any contribution, from code reviewing to direct contribution !
You can :
- Suggest or Improve current code
- Suggest or Add new features
- Suggest or Add new built-in chips
- Any initiative is welcome !

# example usage 

```rust
use std::time::Duration;

use virt_ic::{
    board::{Board, Trace},
    chip::{gates::AndGate, generators::Generator, Chip, ChipBuilder, ChipType},
};

fn main() {
    // create a new board
    let mut board: Board<ChipType> = Board::new();
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
```

# Documentation

Take a look at the [generated documentation](https://docs.rs/virt-ic/).

# Examples

See [examples](https://github.com/VincentFoulon80/virt-ic/tree/master/examples) :
- **pins** : Read and write a set of pins using Pin::read and Pin::write
- **ram** : A simple test of a RAM chip
- **readme** : Same example as provided in this readme
- **save** : Board saving and loading example
- **segment-display** : Simple segment display test
- **sr-latch** : Example of a working SR-Latch
- **test-6502** : Test the Nes6502 CPU with a small program and basic ROM and RAM layout
