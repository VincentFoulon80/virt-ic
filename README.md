# Virtual Integrated Circuits

[Changelog](https://github.com/VincentFoulon80/virt-ic/releases)

This library is a Integrated Circuit Emulator backend that can simulate interractions between multiple chips.

You start by creating a Board, and then you add Traces or Sockets, and then you plug Chips and link Pins together to form a virtual circuit.
You can then run the circuit to emulate the chips and links between them.

This library is a Backend emulator, it means that there is no interface (yet) to create boards. It'll only emulate chips.

## Chips

- Generator
- Buttons
- Logic Gates (And, Or, Not)
- Clocks
- Memory (RAM, ROM)
- CPU (right now there is only one fictional CPU)

# example usage 

```rust
use virt_ic::chip::gates::GateAnd;
use virt_ic::chip::generators::Generator;
use virt_ic::chip::Chip;
use virt_ic::{Board,State};
use std::time::Duration;

fn main() {
    // create a board
    let mut board = Board::new();
    // place sockets with chips on the board
    let gen = board.new_socket_with(Box::new(Generator::new()));
    let and_gate = board.new_socket_with(Box::new(GateAnd::new()));
    // place traces 
    {
        // VCC
        let trc = board.new_trace();
        trc.borrow_mut().connect(gen.borrow_mut().get_pin(Generator::VCC).unwrap());
        trc.borrow_mut().connect(and_gate.borrow_mut().get_pin(GateAnd::VCC).unwrap());
    }
    {
        // GND
        let trc = board.new_trace();
        trc.borrow_mut().connect(gen.borrow_mut().get_pin(Generator::GND).unwrap());
        trc.borrow_mut().connect(and_gate.borrow_mut().get_pin(GateAnd::GND).unwrap());
    }
    {
        // link pin "A&B" to pin "C"
        let trc = board.new_trace();
        trc.borrow_mut().connect(and_gate.borrow_mut().get_pin(GateAnd::A_AND_B).unwrap());
        trc.borrow_mut().connect(and_gate.borrow_mut().get_pin(GateAnd::D).unwrap());
    }
    // run the board to update its state
    // we simulate 1 second segmented by 100 milliseconds
    board.run_during(Duration::from_secs(1), Duration::from_millis(100));
    // test the chip
    println!("ABC:\tA&B\tA&B&C");
    let a_b = and_gate.borrow_mut().get_pin_state(GateAnd::A_AND_B).as_bool();
    println!("000:\t{}\t{}", a_b, and_gate.borrow_mut().get_pin_state(GateAnd::C_AND_D).as_bool());


    // set some pins manually and test the result
    and_gate.borrow_mut().set_pin_state(GateAnd::A, &State::High);
    and_gate.borrow_mut().set_pin_state(GateAnd::B, &State::High);
    and_gate.borrow_mut().set_pin_state(GateAnd::C, &State::Low);
    board.run_during(Duration::from_secs(1), Duration::from_millis(100));
    
    let a_b = and_gate.borrow_mut().get_pin_state(GateAnd::A_AND_B).as_bool();
    println!("110:\t{}\t{}", a_b, and_gate.borrow_mut().get_pin_state(GateAnd::C_AND_D).as_bool());


    and_gate.borrow_mut().set_pin_state(GateAnd::C, &State::High);
    board.run_during(Duration::from_secs(1), Duration::from_millis(100));
    
    let a_b = and_gate.borrow_mut().get_pin_state(GateAnd::A_AND_B).as_bool();
    println!("111:\t{}\t{}", a_b, and_gate.borrow_mut().get_pin_state(GateAnd::C_AND_D).as_bool());


    and_gate.borrow_mut().set_pin_state(GateAnd::A, &State::Low);
    and_gate.borrow_mut().set_pin_state(GateAnd::B, &State::Low);
    and_gate.borrow_mut().set_pin_state(GateAnd::C, &State::High);
    board.run_during(Duration::from_secs(1), Duration::from_millis(100));

    let a_b = and_gate.borrow_mut().get_pin_state(GateAnd::A_AND_B).as_bool();
    println!("001:\t{}\t{}", a_b, and_gate.borrow_mut().get_pin_state(GateAnd::C_AND_D).as_bool());
}
```

# Documentation

Take a look at the [generated documentation](https://docs.rs/virt-ic/).

# Examples

See [examples](https://github.com/VincentFoulon80/virt-ic/tree/master/examples) :
- **readme** : Same example as provided in this readme
- **ram-test** : A simple test of a RAM chip
- **cpu-test** : A simple circuit containing a minimal setup running a CPU processing factorial of 5