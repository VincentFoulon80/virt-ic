use virt_ic::{
    chip::{Pin, PinType},
    State,
};

fn main() {
    let mut a = Pin {
        pin_type: PinType::Output,
        state: State::High,
    };
    let mut b = Pin {
        pin_type: PinType::Output,
        state: State::High,
    };
    let mut c = Pin {
        pin_type: PinType::Output,
        state: State::High,
    };
    let mut d = Pin {
        pin_type: PinType::Output,
        state: State::High,
    };

    dbg!(Pin::read(&[&a, &b, &c, &d]));

    dbg!(Pin::write(&mut [&mut a, &mut b, &mut c, &mut d], 20));

    dbg!(Pin::read(&[&a, &b, &c, &d]));
}
