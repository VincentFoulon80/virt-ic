//! Central Processing Units
use super::super::State;
use super::{Pin, PinType, Chip};
use std::cell::RefCell;
use std::rc::Rc;

/// # A simple example CPU
/// - 4M of address space (12 ADDR pins)
/// - 8-bit IO Pins
/// - 3 data register (Accumulator, B and C)
/// - 2 address registers (H and L forming the full address HL)
/// 
/// # Instructions
/// 
/// On startup or RESET, the CPU will fetch the boot address at 0xFFD and 0xFFE.  
/// The Addresses are stored in this order : MSD, LSD, so 0x0F and 0x12 will create address 0xF12.
/// 
/// The stack pointer will be initialized in the bank specified at 0xFFF.  
/// If 0xFFF contains 0x0E, the CPU will use 0x0E00 to 0x0EFF for his stack.  
/// Note that the bank can't go beyond 0x0F since the CPU only has a 12-bit address space.   
/// 
/// TODO: Implement IRQ  
/// On IRQ, the CPU will fetch the Interrupt code address at 0xFFB and 0xFFC.  
/// When IRQ is triggered, the Address at 0xFFC and 0xFFD will be used as a JSR opcode.  
/// To return to the main code, you'll just need to execute a RTN opcode.  
/// 
/// 
/// # Opcodes
/// MSD\LSD| x0| x1| x2| x3| x4| x5| x6| x7| x8| x9| xA| xB| xC| xD| xE| xF|   |
/// -------|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
/// 0x     |HLT|INA|DEA|INL|DEL|CLC|ADB|ADC|TAB|TBA|TAC|TCA|TAH|THA|TAL|TLA| 0x|
/// 1x     |PHA|PLA|PHL|PLL|CPB|CPC|SUB|SUC|SAL|SAR|   |   |   |   |   |   | 1x|
/// 2x     |JML|JSL|RTN|   |   |   |   |   |   |   |   |   |   |   |   |   | 2x|
/// 3x     |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   | 3x|
/// 4x     |   |   |   |   |   |   |   |   |STA|STB|STC|LDB|LDC|LDA|LDA|LDA| 4x|
/// 5x     |LDA|LDA|LDA|LDA|LDB|LDB|LDC|LDC|STA|STA|STA|STB|STC|   |   |   | 5x|
/// 6x     |CMP|CPB|CPC|   |   |   |   |   |   |   |   |   |   |   |   |   | 6x|
/// 7x     |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   | 7x|
/// 8x     |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   | 8x|
/// 9x     |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   | 9x|
/// Ax     |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   | Ax|
/// Bx     |JMP|JSR|BCF|BNF|BZF|   |   |   |   |   |   |   |   |   |   |   | Bx|
/// Cx     |LDA|STA|   |   |   |   |   |   |   |   |   |   |   |   |   |   | Cx|
/// Dx     |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   | Dx|
/// Ex     |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   | Ex|
/// Fx     |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   | Fx|
/// -      | x0| x1| x2| x3| x4| x5| x6| x7| x8| x9| xA| xB| xC| xD| xE| xF|   |
/// 
/// Opcode|Parameters|Description|
/// ------|----------|-----------|
/// HLT (0x00) | - |Halts the CPU|
/// INA (0x01) | - |Increments the accumulator|
/// DEA (0x02) | - |Decrements the accumulator|
/// INL (0x03) | - |Increments the HL register|
/// DEL (0x04) | - |Decrements the HL register|
/// CLC (0x05) | - |Clear the Carry flag|
/// ADB (0x06) | - |Add B to Accumulator|
/// ADC (0x07) | - |Add C to Accumulator|
/// TAB (0x08) | - |Transfer Accumulator into B register|
/// TBA (0x09) | - |Transfer B register into Accumulator|
/// TAC (0x0A) | - |Transfer Accumulator into C register|
/// TCA (0x0B) | - |Transfer C register into Accumulator|
/// TAH (0x0C) | - |Transfer Accumulator into H register|
/// THA (0x0D) | - |Transfer H register into Accumulator|
/// TAL (0x0E) | - |Transfer Accumulator into L register|
/// TLA (0x0F) | - |Transfer L register into Accumulator|
/// PHA (0x10) | - |Push the accumulator's value in the stack|
/// PLA (0x11) | - |Pull the stack value in the accumulator|
/// PHL (0x12) | - |Push the HL's value in the stack|
/// PLL (0x13) | - |Pull two bytes from the stack in the HL register|
/// CPB (0x14) | - |Compare B with the Accumulator|
/// CPC (0x15) | - |Compare C with the Accumulator|
/// SUB (0x16) | - |Substract B to accumulator|
/// SUC (0x17) | - |Substract C to accumulator|
/// SAL (0x18) | - |Shift Accumulator Left|
/// SAR (0x19) | - |Shift Accumulator Right|
/// INB (0x1A) | - |Increments the B register|
/// DEB (0x1B) | - |Decrements the B register|
/// INC (0x1C) | - |Increments the C register|
/// DEC (0x1D) | - |Decrements the C register|
/// - | - | - |
/// JML (0x20) | - |Jumps to the address HL
/// JSL (0x21) | - |Jumps to subroutine at address HL
/// RTN (0x22) | - |Return from SubRoutine|
/// - | - | - |
/// STA (0x48) | - |Stores the value of accumulator into address [HL]|
/// STB (0x49) | - |Stores the value of B register into address [HL]|
/// STC (0x4A) | - |Stores the value of C register into address [HL]|
/// LDB (0x4B) | - |Loads the value of address [HL] into the B register|
/// LDC (0x4C) | - |Loads the value of address [HL] into the C register|
/// LDA (0x4D) | - |Loads the value of address [HL] into the accumulator|
/// LDA (0x4E) | - |Loads the value of address [HL]+B into the accumulator|
/// LDA (0x4F) | - |Loads the value of address [HL]+C into the accumulator|
/// LDA (0x50) | $1: number |Loads $1 into the accumulator|
/// LDA (0x51) | [$1]: zero page address |Loads the value of address 0x0$1 into the accumulator|
/// LDA (0x52) | $1: number |Loads the value of address H0 + $1 into the accumulator|
/// LDA (0x53) | $1: number |Loads the value of address HL + $1 into the accumulator|
/// LDB (0x54) | $1: number |Loads $1 into the B register|
/// LDB (0x55) | [$1]: zero page address |Loads the value of address 0x0$1 into the B register|
/// LDC (0x56) | $1: number |Loads $1 into the C register|
/// LDC (0x57) | [$1]: zero page address |Loads the value of address 0x0$1 into the C register|
/// STA (0x58) | [$1]: zero page address |Stores the value of Accumulator into address 0x0$1|
/// STA (0x59) | $1: number |Stores the value of Accumulator into address H0 + $1 |
/// STA (0x5A) | $1: number |Stores the value of Accumulator into address HL + $1|
/// STB (0x5B) | [$1]: zero page address |Stores the value of B register into address 0x0$1|
/// STC (0x5C) | [$1]: zero page address |Stores the value of C register into address 0x0$1|
/// - | - | - |
/// CMP (0x60) | $1: number |Compares the accumulator with $1|
/// CPB (0x61) | $1: number |Compares the B register with $1|
/// CPC (0x62) | $1: number |Compares the C register with $1|
/// - | - | - |
/// JMP (0xB0) | $1$2: address| Jumps to the address $1$2
/// JSR (0xB1) | $1$2: address| Jumps to subroutine at address $1$2
/// BCF (0xB2) | $1$2: address| Branch on Carry flag
/// BNF (0xB3) | $1$2: address| Branch on Negative flag
/// BZF (0xB4) | $1$2: address| Branch on Zero flag
/// - | - | - |
/// LDA (0xC0) | $1$2: address| Load the value of address $1$2 in the accumulator
/// STA (0xC1) | $1$2: address| Store the value of accumulator into address $1$2
/// 
/// # diagram
/// IRQ: Interrupt Request (active low)
/// RESET: Reset (active low)
/// R/!W: Read Write mode
/// CLOCK: Clock pin
/// A0-9: Addresses
/// IO0-7: Input/Output
/// ```
///        ---__---
///   A0 --|1   26|-- VCC
///   A1 --|2   25|-- R/!W
///   A2 --|3   24|-- IO7
///   A3 --|4   23|-- IO6
///   A4 --|5   22|-- IO5
///   A5 --|6   21|-- IO4
///   A6 --|7   20|-- IO3
///   A7 --|8   19|-- IO2
///   A8 --|9   18|-- IO1
///   A9 --|10  17|-- IO0
///  A10 --|11  16|-- !IRQ
///  A11 --|12  15|-- !RESET
///  GND --|13  14|-- CLOCK
///        --------
/// ```
pub struct SimpleCPU {
    pin: [Rc<RefCell<Pin>>; 26],
    program_counter: u16,
    accumulator: u8,
    stack_bank: u8,
    stack_pointer: u8,
    reg_b: u8,
    reg_c: u8,
    reg_h: u8,
    reg_l: u8,
    flag_zero: bool,
    flag_neg: bool,
    flag_carry: bool,
    flag_overflow: bool,
    current_opcode: u8,
    param_first: u8,
    param_second: u8,
    microcode_state: u8,
    executing: bool,
    initializing: bool,
    halted: bool
}
impl Default for SimpleCPU {
    fn default() -> Self {
        Self::new()
    }
}
impl std::fmt::Debug for SimpleCPU {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_str(format!("PC: {:03X}\tADR: {:03X}\tIO: {:02X}\tOp: {:02X}\t$1: {:02X}\t$2: {:02X}\tA: {:02X}\tB: {:02X}\tC: {:02X}\tH: {:02X}\tL: {:02X}\tSP: {:02X}\tmc: {}\texec: {}", self.program_counter, self.get_address(), self.get_data(), self.current_opcode, self.param_first, self.param_second, self.accumulator, self.reg_b, self.reg_c, self.reg_h, self.reg_l, self.stack_pointer, self.microcode_state, self.executing).as_str())?;
        Ok(())
    }
}

impl SimpleCPU {
    pub const A0: u8 = 1;
    pub const A1: u8 = 2;
    pub const A2: u8 = 3;
    pub const A3: u8 = 4;
    pub const A4: u8 = 5;
    pub const A5: u8 = 6;
    pub const A6: u8 = 7;
    pub const A7: u8 = 8;
    pub const A8: u8 = 9;
    pub const A9: u8 = 10;
    pub const A10: u8 = 11;
    pub const A11: u8 = 12;
    pub const CLOCK: u8 = 14;
    pub const RESET: u8 = 15;
    pub const IRQ: u8 = 16;
    pub const IO0: u8 = 17;
    pub const IO1: u8 = 18;
    pub const IO2: u8 = 19;
    pub const IO3: u8 = 20;
    pub const IO4: u8 = 21;
    pub const IO5: u8 = 22;
    pub const IO6: u8 = 23;
    pub const IO7: u8 = 24;
    pub const RW: u8 = 25;
    pub const VCC: u8 = 26;
    pub const GND: u8 = 13;

    pub fn new() -> Self {
        SimpleCPU {
            pin: [
                Rc::new(RefCell::new(Pin::new(1, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(2, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(3, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(4, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(5, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(7, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(9, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(10, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(11, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(12, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(14, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(15, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(16, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(17, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(18, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(19, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(20, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(21, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(22, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(23, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(24, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(25, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(26, PinType::Input)))
            ],
            program_counter: 0,
            accumulator: 0,
            stack_bank: 0,
            stack_pointer: 0,
            reg_b: 0,
            reg_c: 0,
            reg_h: 0,
            reg_l: 0,
            flag_zero: false,
            flag_neg: false,
            flag_carry: false,
            flag_overflow: false,
            current_opcode: 0,
            param_first: 0,
            param_second: 0,
            microcode_state: 0,
            executing: false,
            initializing: true,
            halted: false
        }
    }

    fn get_address(&self) -> u16 {
        let mut addr: u16 = 0;
        for i in 0..12 {
            let bit = if self.pin[i].borrow().state == State::High {1} else {0};
            addr += bit << i;
        }
        addr
    }

    fn set_address(&mut self, addr: u16) {
        // mask overflowing addr
        let mut addr = addr;
        if addr > 0xFFF {
            addr = 0;
        }
        // set the address pins
        for i in 0..12 {
            self.pin[i].borrow_mut().state = State::from_u16(addr, i);
        }
    }

    fn get_data(&self) -> u8 {
        let mut addr: u8 = 0;
        for i in 16..24 {
            let bit = if self.pin[i].borrow().state == State::High {1} else {0};
            addr += bit << (i-16);
        }
        addr
    }

    fn set_data(&mut self, data: u8) {
        // set the IO pins
        for i in 0..8 {
            let mut pin = self.pin[i+16].borrow_mut();
            pin.pin_type = PinType::Output;
            pin.state = State::from_u8(data, i);
        }
        // set R/!W pin
        self.pin[24].borrow_mut().state = State::Low;
    }

    fn set_iopin_type(&mut self, pin_type: PinType) {
        // set IO pins
        for i in 0..8 {
            self.pin[i+16].borrow_mut().pin_type = pin_type.clone();
        }
        // set R/!W pin
        self.pin[24].borrow_mut().state = match pin_type {
            PinType::Input => State::High,
            PinType::Output => State::Low,
            PinType::Undefined => State::Undefined
        }
    }

    /// CPU's boot sequence
    fn boot(&mut self){
        match self.microcode_state {
            0 => {
                // initialize registers
                // + query MSB address
                self.program_counter = 0x00;
                self.stack_pointer = 0;
                self.accumulator = 0;
                self.reg_b = 0;
                self.reg_c = 0;
                self.reg_h = 0;
                self.reg_l = 0;
                self.set_address(0xFFD);
            },
            1 => {
                // get first part of starting address
                // + query LSB address
                self.program_counter += (self.get_data() as u16) << 8;
                self.program_counter &= 0xFFF;
                self.set_address(0xFFE);
            },
            2 => {
                // get second part of starting address
                // + query Stack pointer bank
                self.program_counter += self.get_data() as u16;
                self.set_address(0xFFF);
            },
            3 => {
                // get stack bank and stop the init process
                self.stack_bank = self.get_data();
                self.stack_pointer = 0xFF;
                self.initializing = false;
                self.microcode_state = 0xFF;
            },
            _ => {
                self.initializing = false;
                self.microcode_state = 0xFF;
            }
        }
    }

    /// CPU's execution process
    fn execute(&mut self) {
        self.executing = false;
        let mut check_zero = true;

        let push_stack = |myself: &mut SimpleCPU, data: u8| {
            let sp = (((myself.stack_bank as u16) << 8) + myself.stack_pointer as u16) & 0xFFF;
            myself.set_address(sp);
            myself.set_data(data);
            myself.stack_pointer = myself.stack_pointer.wrapping_sub(1);
            if myself.stack_pointer == 0 {
                myself.flag_overflow = true;
            }
        };
        let req_pull_stack = |myself: &mut SimpleCPU| {
            myself.stack_pointer = myself.stack_pointer.wrapping_add(1);
            let sp = (((myself.stack_bank as u16) << 8) + myself.stack_pointer as u16) & 0xFFF;
            if myself.stack_pointer == 0xFF {
                myself.flag_neg = false;
            }
            myself.set_address(sp);
        };

        match self.current_opcode {
            0x00 => { // HLT : Halt
                self.halted = true;
            },
            0x01 => { // INA : Increment Acc
                self.accumulator = self.accumulator.wrapping_add(1);
                if self.accumulator == 0 {
                    self.flag_overflow = true;
                    self.flag_neg = false;
                }
            }
            0x02 => { // DEA : Decrement Acc 
                self.accumulator = self.accumulator.wrapping_sub(1);
                if self.accumulator == 0xFF {
                    self.flag_overflow = false;
                    self.flag_neg = true;
                }
            },
            0x03 => { // INL : Increment HL 
                let mut hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                hl = hl.wrapping_add(1);
                if hl == 0 {
                    self.flag_overflow = true;
                    self.flag_neg = false;
                }
                self.reg_h = (hl >> 8) as u8;
                self.reg_l = (hl & 0xFF) as u8;
            },
            0x04 => { // DEL : Decrement HL 
                let mut hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                hl = hl.wrapping_sub(1);
                if hl == 0xFFFF {
                    self.flag_overflow = false;
                    self.flag_neg = true;
                }
                self.reg_h = (hl >> 8) as u8;
                self.reg_l = (hl & 0xFF) as u8;
            },
            0x05 => { // CLC : Clear Carry
                self.flag_carry = false;
            },
            0x06 => { // ADB : Add B to Acc
                let add = self.accumulator as u16 + self.reg_b as u16;
                if add > u8::MAX as u16 {
                    self.flag_carry = true;
                    self.flag_overflow = true;
                }
                self.accumulator = self.accumulator.wrapping_add(self.reg_b);
            },
            0x07 => { // ADC : Add C to Acc
                let add = self.accumulator as u16 + self.reg_c as u16;
                if add > u8::MAX as u16 {
                    self.flag_carry = true;
                    self.flag_overflow = true;
                }
                self.accumulator = self.accumulator.wrapping_add(self.reg_c);
            },
            0x08 => { // TAB : Transfer Acc > B
                self.reg_b = self.accumulator;
            },
            0x09 => { // TBA : Transfer B > Acc
                self.accumulator = self.reg_b;
            },
            0x0A => { // TAC : Transfer Acc > C
                self.reg_c = self.accumulator;
            },
            0x0B => { // TCA : Transfer C > Acc
                self.accumulator = self.reg_c;
            },
            0x0C => { // TAH : Transfer Acc > H
                self.reg_h = self.accumulator;
            },
            0x0D => { // THA : Transfer H > Acc
                self.accumulator = self.reg_h;
            },
            0x0E => { // TAL : Transfer Acc > L
                self.reg_l = self.accumulator;
            },
            0x0F => { // TLA : Transfer L > Acc
                self.accumulator = self.reg_l;
            },
            0x10 => { // PHA : Push Acc
                push_stack(self, self.accumulator);
            },
            0x11 => { // PLA : Pull Acc
                match self.microcode_state {
                    0 => {
                        req_pull_stack(self);
                        self.executing = true;
                    },
                    1 => {
                        self.accumulator = self.get_data();
                    }
                    _ => {}
                }
            },
            0x12 => { // PHL : Push HL
                match self.microcode_state {
                    0 => {
                        push_stack(self, self.reg_l);
                        self.executing = true;
                    },
                    1 => {
                        push_stack(self, self.reg_h)
                    }
                    _ => {}
                }
            },
            0x13 => { // PLL : Pull HL
                match self.microcode_state {
                    0 => {
                        req_pull_stack(self);
                        self.executing = true;
                    },
                    1 => {
                        self.reg_h = self.get_data();
                        req_pull_stack(self);
                        self.executing = true;
                    },
                    2 => {
                        self.reg_l = self.get_data();
                    },
                    _ => {}
                }
            },
            0x14 => { // CPB : Compare B with Acc
                check_zero = false;
                let compare = self.accumulator as i16 - self.reg_b as i16;
                self.flag_zero = compare == 0;
                self.flag_neg = compare < 0;
            },
            0x15 => { // CPC : Compare C with Acc
                check_zero = false;
                let compare = self.accumulator as i16 - self.reg_c as i16;
                self.flag_zero = compare == 0;
                self.flag_neg = compare < 0;
            },
            0x16 => { // SUB : Substract B to Acc
                let sub = self.accumulator as i16 - self.reg_b as i16;
                self.flag_neg = sub < 0;
                self.accumulator -= self.reg_b;
            },
            0x17 => { // SUC : Substract C with Acc
                let sub = self.accumulator as i16 - self.reg_c as i16;
                self.flag_neg = sub < 0;
                self.accumulator -= self.reg_c;
            },
            0x18 => { // SAL : Shift Acc left
                self.accumulator <<= 1;
            },
            0x19 => { // SAR : Shift Acc Right
                self.accumulator >>= 1;
            }
            0x1A => { // INB : Increment B
                self.reg_b = self.reg_b.wrapping_add(1);
                if self.reg_b == 0 {
                    self.flag_overflow = true;
                    self.flag_neg = false;
                }
            }
            0x1B => { // DEB : Decrement B 
                self.reg_b = self.reg_b.wrapping_sub(1);
                if self.reg_b == 0xFF {
                    self.flag_overflow = false;
                    self.flag_neg = true;
                }
            },
            0x1C => { // INC : Increment C
                self.reg_c = self.reg_c.wrapping_add(1);
                if self.reg_c == 0 {
                    self.flag_overflow = true;
                    self.flag_neg = false;
                }
            }
            0x1D => { // DEC : Decrement C 
                self.reg_c = self.reg_c.wrapping_sub(1);
                if self.reg_c == 0xFF {
                    self.flag_overflow = false;
                    self.flag_neg = true;
                }
            },
            0x20 => { // JML : Jump to HL
                self.program_counter = ((self.reg_h as u16) << 8) + self.reg_l as u16;
            },
            0x21 => { // JSL : Jump Subroutine to HL
                match self.microcode_state {
                    0 => {
                        let addr_l = (self.program_counter & 0xFF) as u8;
                        push_stack(self, addr_l);
                        self.executing = true;
                    },
                    1 => {
                        let addr_h = (self.program_counter >> 8) as u8;
                        push_stack(self, addr_h);
                        self.program_counter = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                    },
                    _ => {}
                }
            },
            0x22 => { // RTN : Return Subroutine
                match self.microcode_state {
                    0 => {
                        req_pull_stack(self);
                        self.executing = true;
                    },
                    1 => {
                        self.program_counter = 0;
                        self.program_counter += (self.get_data() as u16) << 8;
                        self.program_counter &= 0xFFF;
                        req_pull_stack(self);
                        self.executing = true;
                    },
                    2 => {
                        self.program_counter += self.get_data() as u16;
                    }
                    _ => {}
                }
            },
            0x48 => { // STA [HL]: Store Acc in addr [HL]
                let hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                self.set_address(hl);
                self.set_data(self.accumulator);
            },
            0x49 => { // STB [HL]: Store B in addr [HL]
                let hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                self.set_address(hl);
                self.set_data(self.reg_b);
            },
            0x4A => { // STC [HL]: Store C in addr [HL]
                let hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                self.set_address(hl);
                self.set_data(self.reg_c);
            },
            0x4B => { // LDB [HL]: Load value of addr [HL] in B
                match self.microcode_state {
                    0 => {
                        let hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                        self.set_address(hl);
                        self.executing = true;
                    },
                    1 => {
                        self.reg_b = self.get_data();
                    },
                    _ => {}
                }
            },
            0x4C => { // LDC [HL]: Load value of addr [HL] in C
                match self.microcode_state {
                    0 => {
                        let hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                        self.set_address(hl);
                        self.executing = true;
                    },
                    1 => {
                        self.reg_c = self.get_data();
                    },
                    _ => {}
                }
            },
            0x4D => { // LDA [HL]: Load value of addr [HL] in Acc
                match self.microcode_state {
                    0 => {
                        let hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                        self.set_address(hl);
                        self.executing = true;
                    },
                    1 => {
                        self.accumulator = self.get_data();
                    },
                    _ => {}
                }
            },
            0x4E => { // LDA [HL+B]: Load value of addr [HL+B] in Acc
                match self.microcode_state {
                    0 => {
                        let hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                        self.set_address(hl + self.reg_b as u16);
                        self.executing = true;
                    },
                    1 => {
                        self.accumulator = self.get_data();
                    },
                    _ => {}
                }
            },
            0x4F => { // LDA [HL+C]: Load value of addr [HL+C] in Acc
                match self.microcode_state {
                    0 => {
                        let hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                        self.set_address(hl + self.reg_c as u16);
                        self.executing = true;
                    },
                    1 => {
                        self.accumulator = self.get_data();
                    },
                    _ => {}
                }
            },
            0x50 => { // LDA $1 : load $1 in Acc
                self.accumulator = self.param_first;
            },
            0x51 => { // LDA [$1] : Zero page load to Acc
                match self.microcode_state {
                    0 => {
                        self.set_address(self.param_first as u16);
                        self.executing = true;
                    },
                    1 => {
                        self.accumulator = self.get_data();
                    },
                    _ => {}
                }
            },
            0x52 => { // LDA [H$1] : Load [H$1] into Acc
                match self.microcode_state {
                    0 => {
                        let h0 = (self.reg_h as u16) << 8;
                        self.set_address(h0 + self.param_first as u16);
                        self.executing = true;
                    },
                    1 => {
                        self.accumulator = self.get_data();
                    },
                    _ => {}
                }
            },
            0x53 => { // LDA [HL]+$1 : Load [HL]+$1 into Acc
                match self.microcode_state {
                    0 => {
                        let hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                        self.set_address(hl + self.param_first as u16);
                        self.executing = true;
                    },
                    1 => {
                        self.accumulator = self.get_data();
                    },
                    _ => {}
                }
            },
            0x54 => { // LDB $1 : Load $1 into B
                self.reg_b = self.param_first
            }
            0x55 => { // LDB [$1] : Zero page load to B
                match self.microcode_state {
                    0 => {
                        self.set_address(self.param_first as u16);
                        self.executing = true;
                    },
                    1 => {
                        self.reg_b = self.get_data();
                    },
                    _ => {}
                }
            },
            0x56 => { // LDC $1 : Load $1 into C
                self.reg_c = self.param_first
            }
            0x57 => { // LDC [$1] : Zero page load to C
                match self.microcode_state {
                    0 => {
                        self.set_address(self.param_first as u16);
                        self.executing = true;
                    },
                    1 => {
                        self.reg_c = self.get_data();
                    },
                    _ => {}
                }
            },
            0x58 => { // STA [$1] : Zero page store Acc
                self.set_address(self.param_first as u16);
                self.set_data(self.accumulator);
            },
            0x59 => { // STA [H0+$1] : Store Acc to [H$1]
                let h0 = (self.reg_h as u16) << 8;
                self.set_address(h0 + self.param_first as u16);
                self.set_data(self.accumulator);
            },
            0x5A => { // STA [HL+$1] : Store Acc to [HL+$1]
                let hl = ((self.reg_h as u16) << 8) + self.reg_l as u16;
                self.set_address(hl + self.param_first as u16);
                self.set_data(self.accumulator);
            },
            0x5B => { // STB [$1] : Zero page store B
                self.set_address(self.param_first as u16);
                self.set_data(self.reg_b);
            },
            0x5C => { // STC [$1] : Zero page store C
                self.set_address(self.param_first as u16);
                self.set_data(self.reg_c);
            },
            0x60 => { // CMP : compare Acc with $1
                check_zero = false;
                let compare = self.accumulator as i16 - self.param_first as i16;
                self.flag_zero = compare == 0;
                self.flag_neg = compare < 0;
            }
            0x61 => { // CPB : compare B with $1
                check_zero = false;
                let compare = self.reg_b as i16 - self.param_first as i16;
                self.flag_zero = compare == 0;
                self.flag_neg = compare < 0;
            }
            0x62 => { // CPC : compare C with $1
                check_zero = false;
                let compare = self.reg_c as i16 - self.param_first as i16;
                self.flag_zero = compare == 0;
                self.flag_neg = compare < 0;
            }
            0xB0 => { // JMP : Jump to $1$2
                self.program_counter = ((self.param_first as u16) << 8) + self.param_second as u16;
            },
            0xB1 => { // JSR : Jump Subroutine to $1$2
                match self.microcode_state {
                    0 => {
                        let addr_l = (self.program_counter & 0xFF) as u8;
                        push_stack(self, addr_l);
                        self.executing = true;
                    },
                    1 => {
                        let addr_h = (self.program_counter >> 8) as u8;
                        push_stack(self, addr_h);
                        self.program_counter = ((self.param_first as u16) << 8) + self.param_second as u16;
                    },
                    _ => {}
                }
            },
            0xB2 => { // BCF : Branch on Carry flag to $1$2
                if self.flag_carry {
                    self.program_counter = ((self.param_first as u16) << 8) + self.param_second as u16;
                }
            },
            0xB3 => { // BNF : Branch on Negative flag to $1$2
                if self.flag_neg {
                    self.program_counter = ((self.param_first as u16) << 8) + self.param_second as u16;
                }
            },
            0xB4 => { // BZF : Branch on Zero flag to $1$2
                if self.flag_zero {
                    self.program_counter = ((self.param_first as u16) << 8) + self.param_second as u16;
                }
            },
            0xC0 => { // LDA [$1$2] : load [$1$2] into Acc
                match self.microcode_state {
                    0 => {
                        self.set_address(((self.param_first as u16) << 8) + self.param_second as u16);
                        self.executing = true;
                    },
                    1 => {
                        self.accumulator = self.get_data();
                    },
                    _ => {}
                }
            },
            0xC1 => { // STA [$1$2] : store Acc into [$1$2]
                self.set_address(self.param_first as u16);
                self.set_data(self.accumulator);
            }
            _ => {} // do nothing, NOP
        }
        if check_zero {
            self.flag_zero = self.accumulator == 0;
        }
        if !self.executing {
            self.microcode_state = 0xFF;
        }
    }
}
impl Chip for SimpleCPU {
    fn get_pin_qty(&self) -> u8 { 
        26
    }

    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str> { 
        if pin > 0 && pin <= 26 {
            Ok(self.pin[pin as usize-1].clone())
        } else {
            Err("Pin out of bounds")
        }
    }
    fn run(&mut self, _: std::time::Duration) {
        if self.pin[14].borrow().state == State::Low {
            self.initializing = true;
            self.microcode_state = 0;
            self.halted = false;
        }
        // check alimented
        if self.pin[12].borrow().state == State::Low && self.pin[25].borrow().state == State::High {
            if self.pin[13].borrow().state == State::High && !self.halted {
                self.set_iopin_type(PinType::Input);
                if self.initializing {
                    // executes boot sequence
                    self.boot();
                } else if self.executing {
                    // run program
                    self.execute();
                } else {
                    let launch_execution = |myself: &mut SimpleCPU| {
                        // execute opcode with its params
                        myself.executing = true;
                        myself.microcode_state = 0xFF;
                    };
                    // fetch program
                    match self.microcode_state {
                        0 => {
                            // set the program counter as address
                            self.set_address(self.program_counter);
                            self.current_opcode = 0;
                            self.param_first = 0;
                            self.param_second = 0;
                        },
                        1 => {
                            self.current_opcode = self.get_data();
                            self.program_counter += 1;
                            if self.current_opcode >= 0x50 {
                                // opcode need at least 1 parameter
                                self.set_address(self.program_counter);
                            } else {
                                launch_execution(self);
                            }
                        },
                        2 => {
                            if self.current_opcode >= 0x50 {
                                self.param_first = self.get_data();
                                self.program_counter += 1;
                                if self.current_opcode >= 0xB0 {
                                    // opcode need at least 2 parameters
                                    self.set_address(self.program_counter);
                                } else {
                                    launch_execution(self);
                                }
                            }
                        },
                        3 => {
                            if self.current_opcode >= 0xB0 {
                                self.param_second = self.get_data();
                                self.program_counter += 1;
                            }
                            launch_execution(self);
                        },
                        _ => {}
                    }
                    
                }
                //println!("PC: {:03X}\tADR: {:03X}\tIO: {:02X}\tOp: {:02X}\t$1: {:02X}\t$2: {:02X}\tA: {:02X}\tB: {:02X}\tC: {:02X}\tH: {:02X}\tL: {:02X}\tSP: {:02X}\tmc: {}\texec: {}", self.program_counter, self.get_address(), self.get_data(), self.current_opcode, self.param_first, self.param_second, self.accumulator, self.reg_b, self.reg_c, self.reg_h, self.reg_l, self.stack_pointer, self.microcode_state, self.executing);
                self.microcode_state = self.microcode_state.wrapping_add(1);
                if self.program_counter > 0xFFF {
                    self.program_counter = 0;
                }
            }
        } else {
            // turn off every pin
            for i in 0..22 {
                self.pin[i].borrow_mut().state = State::Undefined
            }
            self.initializing = true;
        }
    }
}