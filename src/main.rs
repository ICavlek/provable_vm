use provable_vm::provable_vm::{program_loader::load_program, vm::ProvableVM};

fn main() {
    let program = load_program("./examples/program.prov").expect("Failed to load program");
    let mut vm = ProvableVM::new();
    vm.run_program(&program).expect("Failed to run program");

    let _trace_commitment = vm
        .generate_trace_commitment("./target/program.trace")
        .expect("Failed to create trace commitment");
}
