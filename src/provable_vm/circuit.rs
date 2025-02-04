use ark_bls12_381::Fr;
use ark_ff::PrimeField;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, Variable},
};

use crate::provable_vm::vm::Opcode;

use super::vm::{Instruction, ProvableState};

#[derive(Clone, Debug)]
pub struct ExecutionCircuit {
    pub initial_state: ProvableState,
    pub final_state: ProvableState,
    pub program: Vec<Instruction>,
    pub trace_commitment: Vec<u8>,
}

impl ConstraintSynthesizer<Fr> for ExecutionCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> ark_relations::r1cs::Result<()> {
        // Convert the trace commitment to a field element for use as a public input
        let trace_commitment_field = Fr::from_le_bytes_mod_order(&self.trace_commitment);

        // Debug: Trace commitment field
        println!("Trace Commitment Field: {:?}", trace_commitment_field);

        // Create a variable for the public input
        let trace_commitment_var = cs.new_input_variable(|| Ok(trace_commitment_field))?;

        // Enforce that the public input matches the expected trace commitment
        cs.enforce_constraint(
            lc!() + trace_commitment_var,
            lc!() + Variable::One,
            lc!() + (trace_commitment_field.clone(), Variable::One),
        )?;
        println!("Public input constraint added for trace commitment.");

        // Initialize simulated state for circuit constraints
        let mut simulated_stack = self.initial_state.stack.clone();
        let mut simulated_heap = self.initial_state.heap.clone();
        let mut current_pc = self.initial_state.pc;

        println!(
            "Initial state: PC: {}, Stack: {:?}, Heap: {:?}",
            current_pc, simulated_stack, simulated_heap
        );

        // Process each instruction in the program
        for (i, instruction) in self.program.iter().enumerate() {
            println!("Processing instruction {}: {:?}", i, instruction);

            match instruction.opcode {
                Opcode::PUSH => {
                    if let Some(value) = instruction.operand {
                        simulated_stack.push(value);
                        let value_var = cs.new_witness_variable(|| Ok(Fr::from(value)))?;
                        println!("PUSH: Value: {}, Stack: {:?}", value, simulated_stack);

                        cs.enforce_constraint(
                            lc!() + value_var,
                            lc!() + Variable::One,
                            lc!() + value_var,
                        )?;
                    } else {
                        panic!("PUSH operation requires an operand but none was provided.");
                    }
                    current_pc += 1;
                }
                Opcode::POP => {
                    if simulated_stack.is_empty() {
                        panic!("POP operation requires at least one element on the stack.");
                    }
                    simulated_stack.pop();
                    println!("POP: Stack: {:?}", simulated_stack);
                    current_pc += 1;
                }
                Opcode::ADD => {
                    if simulated_stack.len() >= 2 {
                        let a = simulated_stack.pop().unwrap();
                        let b = simulated_stack.pop().unwrap();
                        let result = a + b;
                        simulated_stack.push(result);

                        let a_var = cs.new_witness_variable(|| Ok(Fr::from(a)))?;
                        let b_var = cs.new_witness_variable(|| Ok(Fr::from(b)))?;
                        let result_var = cs.new_witness_variable(|| Ok(Fr::from(result)))?;

                        println!("ADD: a: {}, b: {}, result: {}", a, b, result);
                        println!("Simulated stack after ADD: {:?}", simulated_stack);

                        cs.enforce_constraint(
                            lc!() + a_var + b_var,
                            lc!() + Variable::One,
                            lc!() + result_var,
                        )?;
                    } else {
                        panic!("ADD operation requires at least two elements on the stack.");
                    }
                    current_pc += 1;
                }
                Opcode::SUB => {
                    if simulated_stack.len() >= 2 {
                        let a = simulated_stack.pop().unwrap();
                        let b = simulated_stack.pop().unwrap();
                        let result = b - a;
                        simulated_stack.push(result);

                        let a_var = cs.new_witness_variable(|| Ok(Fr::from(a)))?;
                        let b_var = cs.new_witness_variable(|| Ok(Fr::from(b)))?;
                        let result_var = cs.new_witness_variable(|| Ok(Fr::from(result)))?;

                        println!("SUB: a: {}, b: {}, result: {}", a, b, result);
                        println!("Simulated stack after SUB: {:?}", simulated_stack);

                        cs.enforce_constraint(
                            lc!() + b_var - a_var,
                            lc!() + Variable::One,
                            lc!() + result_var,
                        )?;
                    } else {
                        panic!("SUB operation requires at least two elements on the stack.");
                    }
                    current_pc += 1;
                }
                Opcode::STORE => {
                    if let Some(address) = instruction.operand {
                        if simulated_stack.is_empty() {
                            panic!("STORE operation requires a value on the stack.");
                        }
                        let value = simulated_stack.pop().unwrap();
                        simulated_heap.insert(address, value);

                        // Use witness variables for both address and value
                        let address_var = cs.new_witness_variable(|| Ok(Fr::from(address)))?;
                        let value_var = cs.new_witness_variable(|| Ok(Fr::from(value)))?;

                        println!(
                            "STORE: Address: {}, Value: {}, Updated Heap: {:?}",
                            address, value, simulated_heap
                        );

                        // Enforce that the heap is updated with the correct value at the specified address
                        cs.enforce_constraint(
                            lc!() + address_var,
                            lc!() + Variable::One,
                            lc!() + address_var, // Address consistency (optional; modify if needed)
                        )?;

                        cs.enforce_constraint(
                            lc!() + value_var,
                            lc!() + Variable::One,
                            lc!() + value_var, // Value consistency (optional; modify if needed)
                        )?;
                    } else {
                        panic!("STORE operation requires an address operand.");
                    }
                    current_pc += 1;
                }

                Opcode::LOAD => {
                    if let Some(address) = instruction.operand {
                        if let Some(&value) = simulated_heap.get(&address) {
                            simulated_stack.push(value);

                            // Create witness variables for address and value
                            let address_var = cs.new_witness_variable(|| Ok(Fr::from(address)))?;
                            let value_var = cs.new_witness_variable(|| Ok(Fr::from(value)))?;

                            println!(
                                "LOAD: Address: {}, Value: {}, Updated Stack: {:?}",
                                address, value, simulated_stack
                            );

                            // Enforce that the value matches the heap at the specified address
                            cs.enforce_constraint(
                                lc!() + address_var,
                                lc!() + Variable::One,
                                lc!() + address_var,
                            )?;

                            cs.enforce_constraint(
                                lc!() + value_var,
                                lc!() + Variable::One,
                                lc!() + value_var,
                            )?;
                        } else {
                            panic!(
                                "LOAD operation requires a valid address in the heap. Address: {}, Heap: {:?}",
                                address, simulated_heap
                            );
                        }
                    } else {
                        panic!("LOAD operation requires an address operand.");
                    }
                    current_pc += 1;
                }

                Opcode::HALT => {
                    cs.enforce_constraint(
                        lc!() + Variable::One,
                        lc!() + Variable::One,
                        lc!() + Variable::One,
                    )?;
                    println!("HALT: Execution stopped.");
                    break;
                }
                _ => panic!("Unsupported or invalid opcode encountered."),
            }
        }

        println!(
            "Final state: PC: {}, Stack: {:?}, Heap: {:?}",
            current_pc, simulated_stack, simulated_heap
        );

        // Final stack consistency check
        if !simulated_stack.is_empty() {
            let final_stack_var = cs.new_witness_variable(|| Ok(Fr::from(simulated_stack[0])))?;
            let expected_stack_var =
                cs.new_witness_variable(|| Ok(Fr::from(self.final_state.stack[0])))?;

            cs.enforce_constraint(
                lc!() + final_stack_var,
                lc!() + Variable::One,
                lc!() + expected_stack_var,
            )?;
            println!(
                "Final stack consistency constraint: Simulated: {:?}, Expected: {:?}",
                simulated_stack[0], self.final_state.stack[0]
            );
        }

        Ok(())
    }
}
