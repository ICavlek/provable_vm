use ark_bls12_381::Bls12_381;
use ark_groth16::Groth16;
use ark_snark::CircuitSpecificSetupSNARK;
use provable_vm::provable_vm::{
    circuit::ExecutionCircuit, program_loader::load_program, vm::ProvableVM, zk_proof,
};
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

fn main() {
    let program = load_program("./examples/program.prov").expect("Failed to load program");
    let mut vm = ProvableVM::new();
    vm.run_program(&program).expect("Failed to run program");

    let trace_commitment = vm
        .generate_trace_commitment("./target/program.trace")
        .expect("Failed to create trace commitment");

    let circuit = ExecutionCircuit {
        initial_state: vm.trace.first().unwrap().clone(),
        final_state: vm.trace.last().unwrap().clone(),
        program: program.clone(),
        trace_commitment,
    };

    let mut rng = ChaCha20Rng::from_entropy();
    let (pk, _vk) = Groth16::<Bls12_381>::setup(circuit.clone(), &mut rng).unwrap();

    let proof_file = "./target/program.proof";
    zk_proof::generate_proof(&pk, circuit, proof_file).expect("Failed to generate proof");
}
