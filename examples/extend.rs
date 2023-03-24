use std::time::Duration;

use custom_chip::MyCustomChip;
use virt_ic::{
    board::{Board, Trace},
    chip::{gates::AndGate, generators::Generator, Chip, ChipBuilder, ChipSet},
    impl_chip_type,
};

mod custom_chip {
    use std::time::Duration;

    use virt_ic::{
        chip::{ChipBuilder, ChipRunner, Pin, PinId, PinType},
        generate_chip, State,
    };

    use crate::CustomChipSet;

    #[derive(Debug, Clone, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct MyCustomChip {
        pub vcc: Pin,
        pub gnd: Pin,
        pub a: Pin,
        pub na: Pin,
    }

    impl MyCustomChip {
        pub const VCC: PinId = 1;
        pub const GND: PinId = 2;
        pub const A: PinId = 3;
        pub const NA: PinId = 4;
    }

    impl ChipBuilder<CustomChipSet> for MyCustomChip {
        fn build() -> CustomChipSet {
            CustomChipSet::MyCustomChip(MyCustomChip {
                vcc: Pin::from(PinType::Input),
                gnd: Pin::from(PinType::Output),
                a: Pin::from(PinType::Input),
                na: Pin::from(PinType::Output),
            })
        }
    }

    generate_chip!(
        MyCustomChip,
        vcc: MyCustomChip::VCC,
        gnd: MyCustomChip::GND,
        a: MyCustomChip::A,
        na: MyCustomChip::NA
    );

    impl ChipRunner for MyCustomChip {
        fn run(&mut self, _: Duration) {
            if self.vcc.state.as_logic(3.3) == State::High {
                self.gnd.state = State::Low;
                self.na.state = State::from(!bool::from(self.a.state.as_logic(3.3)));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CustomChipSet {
    MyCustomChip(MyCustomChip),
    Builtin(ChipSet),
}

impl_chip_type!(CustomChipSet: (MyCustomChip, Builtin));

impl From<ChipSet> for CustomChipSet {
    fn from(value: ChipSet) -> Self {
        CustomChipSet::Builtin(value)
    }
}

fn main() {
    // create a new board
    let mut board: Board<CustomChipSet> = Board::new();
    // place an AND gate to the board
    let and_gate = board.register_chip(AndGate::build().into());
    // also place a generator
    let vcc = board.register_chip(ChipSet::from(Generator::build()).into());
    let gnd = board
        .register_chip(ChipSet::from(Generator::build().with_state(virt_ic::State::Low)).into());

    let custom = board.register_chip(MyCustomChip::build());

    // Connect the AndGate's VCC, A and B pins with the Generator
    let mut trace = Trace::new();
    trace.connect(vcc, Generator::OUT);
    trace.connect(and_gate, AndGate::VCC);
    trace.connect(custom, MyCustomChip::VCC);
    trace.connect(custom, MyCustomChip::A);
    trace.connect(and_gate, AndGate::B);
    let trace_vcc = board.register_trace(trace);

    board.connect(custom, MyCustomChip::NA, and_gate, AndGate::A);

    // Alternative way to connect chips via board, connect GND pins
    let trace_gnd = board.register_trace(Trace::from(vec![
        (gnd, Generator::OUT),
        (and_gate, AndGate::GND),
        (custom, MyCustomChip::GND),
    ]));

    // simulate the board for 10ms
    board.run_realtime(Duration::from_millis(10));

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
        t.disconnect(custom, MyCustomChip::A)
    }
    if let Some(t) = board.get_trace_mut(&trace_gnd) {
        t.connect(custom, MyCustomChip::A)
    }

    // simulate the board for another 10ms
    board.run_realtime(Duration::from_millis(10));

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
