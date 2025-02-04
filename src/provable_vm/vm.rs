use std::collections::HashMap;

use eyre::Result;
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProvableState {
    pub pc: u32,
    pub stack: Vec<u32>,
    pub heap: HashMap<u32, u32>,
    pub flags: u8,
}

pub struct ProvableVM {
    pub pc: u32,
    pub stack: Vec<u32>,
    pub heap: HashMap<u32, u32>,
    pub flags: u8,
    pub trace: Vec<ProvableState>,
}

impl ProvableVM {
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: Vec::new(),
            heap: HashMap::new(),
            flags: 0,
            trace: Vec::new(),
        }
    }

    pub fn run_program(&self, program: &[Instruction]) -> Result<()> {
        while let Some(program_instruction) = program.get(self.pc as usize) {
            if !self.execute_instruction(program_instruction)? {
                break;
            }
        }
        Ok(())
    }

    fn execute_instruction(&self, _instruction: &Instruction) -> Result<bool> {
        Ok(false)
    }
}
