use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

    pub fn run_program(&mut self, program: &[Instruction]) -> Result<()> {
        while let Some(program_instruction) = program.get(self.pc as usize) {
            self.trace.push(self.capture_state());
            if !self.execute_instruction(program_instruction)? {
                break;
            }
        }
        self.trace.push(self.capture_state());
        Ok(())
    }

    pub fn generate_trace_commitment(&self, trace_file: &str) -> Result<Vec<u8>> {
        let mut hasher = Sha256::new();

        for state in &self.trace {
            let serialized = bincode::serialize(state)?;
            hasher.update(serialized);
        }

        let hash = hasher.finalize();
        let hex_hash = hex::encode(&hash);

        let mut file = File::create(trace_file)?;
        writeln!(file, "{}", hex_hash)?;

        Ok(hash.to_vec())
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<bool> {
        match instruction.opcode {
            Opcode::PUSH => {
                if let Some(value) = instruction.operand {
                    self.stack.push(value);
                } else {
                    return Err(eyre!("PUSH requires an operand"));
                }
            }
            Opcode::POP => {
                self.stack
                    .pop()
                    .ok_or(eyre!("POP requires at least one element on the stack"))?;
            }
            Opcode::ADD => {
                let a = self
                    .stack
                    .pop()
                    .ok_or(eyre!("ADD requires two elements on the stack"))?;
                let b = self
                    .stack
                    .pop()
                    .ok_or(eyre!("ADD requires two elements on the stack"))?;
                self.stack.push(a + b);
            }
            Opcode::SUB => {
                let a = self
                    .stack
                    .pop()
                    .ok_or(eyre!("SUB requires two elements on the stack"))?;
                let b = self
                    .stack
                    .pop()
                    .ok_or(eyre!("SUB requires two elements on the stack"))?;
                self.stack.push(
                    b.checked_sub(a)
                        .ok_or(eyre!("SUB resulted in an underflow"))?,
                );
            }
            Opcode::LOAD => {
                let addr = instruction
                    .operand
                    .ok_or(eyre!("LOAD requires an address operand"))?;
                let value = *self
                    .heap
                    .get(&addr)
                    .ok_or(eyre!("LOAD failed: address {} not found", addr))?;
                self.stack.push(value);
            }
            Opcode::STORE => {
                let addr = instruction
                    .operand
                    .ok_or(eyre!("STORE requires an address operand"))?;
                let value = self
                    .stack
                    .pop()
                    .ok_or(eyre!("STORE requires a value on the stack"))?;
                self.heap.insert(addr, value);
            }
            Opcode::HALT => return Ok(false),
            _ => return Err(eyre!("Unsupported opcode: {:?}", instruction.opcode)),
        }

        self.pc += 1;
        Ok(true)
    }

    fn capture_state(&self) -> ProvableState {
        ProvableState {
            pc: self.pc,
            stack: self.stack.clone(),
            heap: self.heap.clone(),
            flags: self.flags,
        }
    }
}
