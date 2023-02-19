use super::opcodes::{Opcode, ParseError};

pub struct Assembler;

impl Assembler {
    pub fn assemble(code: &[Opcode]) -> Result<Vec<u8>, ParseError> {
        let mut payload = vec![];
        for opcode in code.iter() {
            payload.append(&mut Vec::<u8>::try_from(*opcode)?);
        }
        Ok(payload)
    }

    // pub fn disassemble(payload: &[u8]) -> Vec<(u16, Opcode)> {
    //     let mut operations = vec![];
    //     let mut state = MicrocodeState::Fetch;
    //     let mut op_index = 0;
    //     for (index, byte) in payload.iter().enumerate() {
    //         state = state.advance(*byte);
    //         if matches!(state, MicrocodeState::Execute(_, _, _, _, _)) {
    //             operations.push((op_index, Operation::from(&state)));
    //             state = MicrocodeState::Fetch;
    //             op_index = index as u16 + 1
    //         }
    //     }
    //     // process incomplete operation
    //     operations.push((op_index, Operation::from(&state)));
    //     operations
    // }

    // pub fn from_code(code: &str) -> (Vec<Opcode>, Vec<(usize, OperationParseError)>) {
    //     let mut operations = vec![];
    //     let mut errors = vec![];
    //     for (line_nb, line) in code.split('\n').enumerate() {
    //         if !line.is_empty() {
    //             if let Ok(byte) = utils::parse_u8(line) {
    //                 operations.push(Operation::Raw(byte));
    //             } else {
    //                 match Operation::from_str(line) {
    //                     Ok(op) => operations.push(op),
    //                     Err(err) => {
    //                         errors.push((line_nb, err));
    //                         operations.push(Operation::None)
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     (operations, errors)
    // }
}
