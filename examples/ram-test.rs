use virt_ic::chip::memory::Ram256B;
use virt_ic::chip::Chip;
use virt_ic::State;

fn main() {
    // create a new Ram chip
    let mut ram = Ram256B::new();

    // initial run to initalize pins
    ram.run(std::time::Duration::from_secs(1));
    println!("non alimented:\n{}", ram.to_string());

    // set the VCC and GND pins to power the whip
    ram.set_pin_state(Ram256B::GND, &State::Low);
    ram.set_pin_state(Ram256B::VCC, &State::High);
    // run the chip to update the alimented state
    ram.run(std::time::Duration::from_secs(1));
    println!("alimented:\n{}", ram.to_string());

    // Set Address pins to 0x00
    ram.set_pin_state(Ram256B::A0, &State::Low);
    ram.set_pin_state(Ram256B::A1, &State::Low);
    ram.set_pin_state(Ram256B::A2, &State::Low);
    ram.set_pin_state(Ram256B::A3, &State::Low);
    ram.set_pin_state(Ram256B::A4, &State::Low);
    ram.set_pin_state(Ram256B::A5, &State::Low);
    ram.set_pin_state(Ram256B::A6, &State::Low);
    ram.set_pin_state(Ram256B::A7, &State::Low);
    // Set Data pins to 0x00
    ram.set_pin_state(Ram256B::IO0, &State::Low);
    ram.set_pin_state(Ram256B::IO1, &State::Low);
    ram.set_pin_state(Ram256B::IO2, &State::Low);
    ram.set_pin_state(Ram256B::IO3, &State::Low);
    ram.set_pin_state(Ram256B::IO4, &State::Low);
    ram.set_pin_state(Ram256B::IO5, &State::Low);
    ram.set_pin_state(Ram256B::IO6, &State::Low);
    ram.set_pin_state(Ram256B::IO7, &State::Low);
    // Set the control pins to enable writing
    ram.set_pin_state(Ram256B::CS, &State::Low);
    ram.set_pin_state(Ram256B::WE, &State::Low);
    ram.set_pin_state(Ram256B::OE, &State::High);
    // run the chip to update its state
    ram.run(std::time::Duration::from_secs(1));
    println!("write first byte:\n{}", ram.to_string());

    // Set the Address to 0x01 and Data to 0x02
    ram.set_pin_state(Ram256B::A0, &State::High);
    ram.set_pin_state(Ram256B::IO1, &State::High);
    // run the chip to update its state
    ram.run(std::time::Duration::from_secs(1));
    println!("write second byte:\n{}", ram.to_string());

    // Set the control pins to read mode
    ram.set_pin_state(Ram256B::CS, &State::Low);
    ram.set_pin_state(Ram256B::WE, &State::High);
    ram.set_pin_state(Ram256B::OE, &State::Low);

    // run the chip and read its IO pins
    ram.run(std::time::Duration::from_secs(1));
    println!(
        "read second byte:\n{}{}{}{}{}{}{}{}",
        ram.get_pin_state(Ram256B::IO7).as_u8(),
        ram.get_pin_state(Ram256B::IO6).as_u8(),
        ram.get_pin_state(Ram256B::IO5).as_u8(),
        ram.get_pin_state(Ram256B::IO4).as_u8(),
        ram.get_pin_state(Ram256B::IO3).as_u8(),
        ram.get_pin_state(Ram256B::IO2).as_u8(),
        ram.get_pin_state(Ram256B::IO1).as_u8(),
        ram.get_pin_state(Ram256B::IO0).as_u8()
    );

    // change the address to 0x02
    ram.set_pin_state(Ram256B::A0, &State::Low);
    ram.set_pin_state(Ram256B::A1, &State::High);
    // run the chip and read its IO pins
    ram.run(std::time::Duration::from_secs(1));
    println!(
        "read third byte:\n{}{}{}{}{}{}{}{}",
        ram.get_pin_state(Ram256B::IO7).as_u8(),
        ram.get_pin_state(Ram256B::IO6).as_u8(),
        ram.get_pin_state(Ram256B::IO5).as_u8(),
        ram.get_pin_state(Ram256B::IO4).as_u8(),
        ram.get_pin_state(Ram256B::IO3).as_u8(),
        ram.get_pin_state(Ram256B::IO2).as_u8(),
        ram.get_pin_state(Ram256B::IO1).as_u8(),
        ram.get_pin_state(Ram256B::IO0).as_u8()
    );
}
