# Virtual Integrated Circuits

[Changelog](https://github.com/VincentFoulon80/virt-ic/releases)

This library is a Integrated Circuit Emulator backend that can simulate interactions between multiple chips.

Note that for now there is only Digital circuit emulation, Analog signals will be implemented later.

You start by creating a Board, and then you add Traces or Sockets, and then you plug Chips and link Pins together to form a virtual circuit.
You can then run the circuit to emulate the chips and links between them.

This library is a Backend emulator, it means that there is no GUI (yet) to create boards.

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
    let chip = board.get_chip(&and_gate);
    println!(
        "A={:?}, \tB={:?}, \tA&B={:?}",
        chip.get_pin(AndGate::A).map(|p| p.state),
        chip.get_pin(AndGate::B).map(|p| p.state),
        chip.get_pin(AndGate::AB).map(|p| p.state)
    );

    // disconnect AndGate's pin B from VCC and connect it instead to GND
    board
        .get_trace_mut(&trace_vcc)
        .disconnect(and_gate, AndGate::B);
    board
        .get_trace_mut(&trace_gnd)
        .connect(and_gate, AndGate::B);

    // simulate the board for another 10ms
    board.run(Duration::from_millis(10));

    // check the results
    let chip = board.get_chip(&and_gate);
    println!(
        "A={:?}, \tB={:?}, \tA&B={:?}",
        chip.get_pin(AndGate::A).map(|p| p.state),
        chip.get_pin(AndGate::B).map(|p| p.state),
        chip.get_pin(AndGate::AB).map(|p| p.state)
    );
}
```

# Documentation

Take a look at the [generated documentation](https://docs.rs/virt-ic/).

# Examples

See [examples](https://github.com/VincentFoulon80/virt-ic/tree/master/examples) :
- **pins** : Read and write a set of pins using Pin::read and Pin::write
- **readme** : Same example as provided in this readme
- **ram** : A simple test of a RAM chip
- **save** : Board saving and loading example
