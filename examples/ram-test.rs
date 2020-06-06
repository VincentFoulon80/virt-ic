use virt_ic::chip::memory::Ram256B;
use virt_ic::chip::Chip;
use virt_ic::State;

fn main() {
    let mut ram = Ram256B::new();

    ram.run(std::time::Duration::from_secs(1));

    println!("non alimented:\n{}", ram.to_string());

    
    ram.get_pin(11).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(22).unwrap().borrow_mut().state = State::High;

    ram.run(std::time::Duration::from_secs(1));

    println!("alimented:\n{}", ram.to_string());


    ram.get_pin(4).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(5).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(6).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(7).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(8).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(9).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(10).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(12).unwrap().borrow_mut().state = State::Low;

    ram.get_pin(13).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(14).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(15).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(16).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(17).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(18).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(19).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(20).unwrap().borrow_mut().state = State::Low;

    ram.get_pin(1).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(2).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(3).unwrap().borrow_mut().state = State::High;

    ram.run(std::time::Duration::from_secs(1));

    println!("write first byte:\n{}", ram.to_string());


    ram.get_pin(4).unwrap().borrow_mut().state = State::High;

    ram.get_pin(14).unwrap().borrow_mut().state = State::High;

    ram.run(std::time::Duration::from_secs(1));

    println!("write second byte:\n{}", ram.to_string());


    ram.get_pin(1).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(2).unwrap().borrow_mut().state = State::High;
    ram.get_pin(3).unwrap().borrow_mut().state = State::Low;

    ram.run(std::time::Duration::from_secs(1));

    println!("read second byte:\n{}{}{}{}{}{}{}{}", 
        if ram.get_pin(20).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(19).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(18).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(17).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(16).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(15).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(14).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(13).unwrap().borrow().state == State::High {1} else {0}
    );


    ram.get_pin(4).unwrap().borrow_mut().state = State::Low;
    ram.get_pin(5).unwrap().borrow_mut().state = State::High;

    ram.run(std::time::Duration::from_secs(1));

    println!("read third byte:\n{}{}{}{}{}{}{}{}", 
        if ram.get_pin(20).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(19).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(18).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(17).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(16).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(15).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(14).unwrap().borrow().state == State::High {1} else {0},
        if ram.get_pin(13).unwrap().borrow().state == State::High {1} else {0}
    );
    
    println!("\n\n");
}