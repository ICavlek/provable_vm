use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: Option<u32>,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Opcode {
    PUSH = 1,
    POP = 2,
    ADD = 3,
    SUB = 4,
    JMP = 5,
    JZ = 6,
    LOAD = 7,
    STORE = 8,
    HALT = 9,
}
