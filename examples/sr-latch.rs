use std::time::Duration;

use virt_ic::{
    board::{Board, Trace},
    chip::{gates::NandGate, generators::Generator, inputs::Button, Chip, ChipBuilder, ChipType},
};

/// ```txt
///    /
/// --o  -|¯¯¯\
/// RESET |NAND|----> Q
///     ,-|___/  |
///      ⟍      ⟋
///        ⟍  ⟋
///         ⟋⟍
///       ⟋    ⟍
///     ⟋        \
///     `-|¯¯¯\   |
///  SET  |NAND|----> !Q
/// --o  -|___/
///    \
/// ```
fn main() {
    let mut board: Board<ChipType> = Board::new();

    let nand = board.register_chip(NandGate::build());

    let gen = board.register_chip(Generator::build().into());

    let set_btn = board.register_chip(Button::build());
    let reset_btn = board.register_chip(Button::build());

    board.register_trace(Trace::from(vec![
        (gen, Generator::OUT),
        (nand, NandGate::VCC),
        (set_btn, Button::I),
        (reset_btn, Button::I),
    ]));

    board.connect(set_btn, Button::O, nand, NandGate::C);
    board.connect(reset_btn, Button::O, nand, NandGate::A);

    board.connect(nand, NandGate::AB, nand, NandGate::D);
    board.connect(nand, NandGate::CD, nand, NandGate::B);

    board.run(Duration::from_millis(1));

    println!("initial state");
    dbg!(board.get_chip(&nand).and_then(|c| c.get_pin(NandGate::AB)));

    if let Some(ChipType::Button(btn)) = board.get_chip_mut(&set_btn) {
        btn.press();
    }
    board.run(Duration::from_millis(1));
    if let Some(ChipType::Button(btn)) = board.get_chip_mut(&set_btn) {
        btn.release();
    }
    board.run(Duration::from_millis(1));

    println!("after set");
    dbg!(board.get_chip(&nand).and_then(|c| c.get_pin(NandGate::AB)));

    if let Some(ChipType::Button(btn)) = board.get_chip_mut(&reset_btn) {
        btn.press();
    }
    board.run(Duration::from_millis(1));
    if let Some(ChipType::Button(btn)) = board.get_chip_mut(&reset_btn) {
        btn.release();
    }
    board.run(Duration::from_millis(1));

    println!("after reset");
    dbg!(board.get_chip(&nand).and_then(|c| c.get_pin(NandGate::AB)));
}
