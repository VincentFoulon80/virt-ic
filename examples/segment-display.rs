use std::time::Duration;

use virt_ic::{
    board::{Board, Trace},
    chip::{
        generators::Generator,
        inputs::Button,
        outputs::{SegmentDisplay, SevenSegmentsDecoder},
        ChipBuilder, ChipType,
    },
};

fn main() {
    let mut board: Board<ChipType> = Board::new();

    let seg_dec = board.register_chip(SevenSegmentsDecoder::build());

    let display = board.register_chip(SegmentDisplay::build());

    let gen = board.register_chip(Generator::build().into());

    let btn_a = board.register_chip(Button::build());
    let btn_b = board.register_chip(Button::build());
    let btn_c = board.register_chip(Button::build());
    let btn_d = board.register_chip(Button::build());

    board.register_trace(Trace::from(vec![
        (gen, Generator::OUT),
        (seg_dec, SevenSegmentsDecoder::VCC),
        (seg_dec, SevenSegmentsDecoder::BI),
        (display, SegmentDisplay::VCC),
        (btn_a, Button::I),
        (btn_b, Button::I),
        (btn_c, Button::I),
        (btn_d, Button::I),
    ]));

    board.connect(btn_a, Button::O, seg_dec, SevenSegmentsDecoder::IA);
    board.connect(btn_b, Button::O, seg_dec, SevenSegmentsDecoder::IB);
    board.connect(btn_c, Button::O, seg_dec, SevenSegmentsDecoder::IC);
    board.connect(btn_d, Button::O, seg_dec, SevenSegmentsDecoder::ID);

    board.connect(
        seg_dec,
        SevenSegmentsDecoder::OA,
        display,
        SegmentDisplay::A,
    );
    board.connect(
        seg_dec,
        SevenSegmentsDecoder::OB,
        display,
        SegmentDisplay::B,
    );
    board.connect(
        seg_dec,
        SevenSegmentsDecoder::OC,
        display,
        SegmentDisplay::C,
    );
    board.connect(
        seg_dec,
        SevenSegmentsDecoder::OD,
        display,
        SegmentDisplay::D,
    );
    board.connect(
        seg_dec,
        SevenSegmentsDecoder::OE,
        display,
        SegmentDisplay::E,
    );
    board.connect(
        seg_dec,
        SevenSegmentsDecoder::OF,
        display,
        SegmentDisplay::F,
    );
    board.connect(
        seg_dec,
        SevenSegmentsDecoder::OG,
        display,
        SegmentDisplay::G,
    );

    board.run_realtime(Duration::from_millis(100));

    if let ChipType::SegmentDisplay(display) = board.get_chip(&display) {
        println!("{}:\n{}", display.as_char(), display.to_string());
    }

    if let ChipType::Button(a) = board.get_chip_mut(&btn_a) {
        a.press();
    }

    board.run_realtime(Duration::from_millis(100));

    if let ChipType::SegmentDisplay(display) = board.get_chip(&display) {
        println!("{}:\n{}", display.as_char(), display.to_string());
    }

    if let ChipType::Button(a) = board.get_chip_mut(&btn_a) {
        a.release();
    }
    if let ChipType::Button(b) = board.get_chip_mut(&btn_b) {
        b.press();
    }

    board.run_realtime(Duration::from_millis(100));

    if let ChipType::SegmentDisplay(display) = board.get_chip(&display) {
        println!("{}:\n{}", display.as_char(), display.to_string());
    }
    if let ChipType::Button(b) = board.get_chip_mut(&btn_b) {
        b.release();
    }
    if let ChipType::Button(c) = board.get_chip_mut(&btn_c) {
        c.press();
    }

    board.run_realtime(Duration::from_millis(100));

    if let ChipType::SegmentDisplay(display) = board.get_chip(&display) {
        println!("{}:\n{}", display.as_char(), display.to_string());
    }

    if let ChipType::Button(c) = board.get_chip_mut(&btn_c) {
        c.release();
    }
    if let ChipType::Button(d) = board.get_chip_mut(&btn_d) {
        d.press();
    }

    board.run_realtime(Duration::from_millis(100));

    if let ChipType::SegmentDisplay(display) = board.get_chip(&display) {
        println!("{}:\n{}", display.as_char(), display.to_string());
    }
}
