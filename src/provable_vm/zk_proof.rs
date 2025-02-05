use std::fs::File;

use ark_bls12_381::Bls12_381;
use ark_groth16::{r1cs_to_qap::LibsnarkReduction, Groth16, ProvingKey};
use ark_serialize::CanonicalSerialize;
use ark_snark::SNARK;
use eyre::Result;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

use super::circuit::ExecutionCircuit;

pub fn generate_proof(
    pk: &ProvingKey<Bls12_381>,
    circuit: ExecutionCircuit,
    proof_file: &str,
) -> Result<()> {
    let mut rng = ChaCha20Rng::from_entropy();
    let proof = Groth16::<Bls12_381, LibsnarkReduction>::prove(pk, circuit, &mut rng)?;
    println!("{:#?}", proof);

    let mut file = File::create(proof_file)?;
    proof.serialize_compressed(&mut file)?;

    println!("Proof written to '{}'", proof_file);
    Ok(())
}
