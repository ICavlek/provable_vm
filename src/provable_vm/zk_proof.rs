use std::fs::File;

use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{r1cs_to_qap::LibsnarkReduction, Groth16, Proof, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
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

pub fn verify_proof(
    proof_file: &str,
    vk: &VerifyingKey<Bls12_381>,
    public_input: &[Fr],
) -> Result<()> {
    let proof = File::open(proof_file)
        .ok()
        .and_then(|mut file| Proof::deserialize_compressed(&mut file).ok())
        .expect("Failed to open proof file");

    Groth16::<Bls12_381>::verify(vk, public_input, &proof)?;
    println!("Successfully verified proof");
    Ok(())
}
