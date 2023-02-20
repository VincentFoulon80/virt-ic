use std::time::Duration;

use virt_ic::{
    chip::{memories::Ram256B, ChipBuilder, ChipRunner, ChipType, Pin},
    State,
};

fn main() {
    if let ChipType::Ram256B(mut ram) = Ram256B::build() {
        ram.vcc.state = State::High;

        ram.run(Duration::from_millis(10));
        println!("{}", ram.to_string());

        ram.cs.state = State::Low;
        ram.oe.state = State::High;
        ram.we.state = State::Low;
        for i in 0..256 {
            Pin::write(
                &mut [
                    &mut ram.a0,
                    &mut ram.a1,
                    &mut ram.a2,
                    &mut ram.a3,
                    &mut ram.a4,
                    &mut ram.a5,
                    &mut ram.a6,
                    &mut ram.a7,
                ],
                i,
            );
            Pin::write(
                &mut [
                    &mut ram.io0,
                    &mut ram.io1,
                    &mut ram.io2,
                    &mut ram.io3,
                    &mut ram.io4,
                    &mut ram.io5,
                    &mut ram.io6,
                    &mut ram.io7,
                ],
                i,
            );
            ram.run(Duration::from_millis(1));
        }

        println!("{}", ram.to_string());

        ram.we.state = State::High;
        ram.oe.state = State::Low;
        Pin::write(
            &mut [
                &mut ram.a0,
                &mut ram.a1,
                &mut ram.a2,
                &mut ram.a3,
                &mut ram.a4,
                &mut ram.a5,
                &mut ram.a6,
                &mut ram.a7,
            ],
            0x5A,
        );
        ram.run(Duration::from_millis(1));
        println!(
            "0x5A => 0x{:0x}",
            Pin::read(&[
                &ram.io0, &ram.io1, &ram.io2, &ram.io3, &ram.io4, &ram.io5, &ram.io6, &ram.io7,
            ])
        );
    }
}
