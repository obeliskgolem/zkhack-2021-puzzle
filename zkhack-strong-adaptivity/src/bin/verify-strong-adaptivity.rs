#![allow(unused, unreachable_code)]
use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use prompt::{puzzle, welcome};
use strong_adaptivity::verify;
use strong_adaptivity::PUZZLE_DESCRIPTION;
use strong_adaptivity::{data::puzzle_data, Instance, Proof};

use ark_ff::UniformRand;
use std::str::FromStr;
use strong_adaptivity::utils::b2s_hash_to_field;
use strong_adaptivity::ProofCommitment;
use strong_adaptivity::ProofResponse;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let ck = puzzle_data();

    let mut rng = rand::thread_rng();
    let mut rng = &mut rng;

    let r = Fr::from_str("1").unwrap();
    let r_alt = Fr::from_str("2").unwrap();
    let rho = Fr::rand(rng);
    let tau = Fr::rand(rng);

    let ck = puzzle_data();

    let (comm_rho, rho) = ck.commit_with_rng(r, rng);
    let (comm_tau, tau) = ck.commit_with_rng(r_alt, rng);
    let commitment = ProofCommitment { comm_rho, comm_tau };

    let challenge = b2s_hash_to_field(&(ck, commitment));

    let a1 = Fr::from_str("100").unwrap();
    let a2 = ((challenge * a1) + (r - r_alt)) / challenge;

    let r1 = Fr::from_str("200").unwrap();
    let r2 = ((rho - tau) + (challenge * r1)) / challenge;

    let comm_1 = ck.commit_with_explicit_randomness(a1, r1);
    let comm_2 = ck.commit_with_explicit_randomness(a2, r2);
    let ins = Instance {
        comm_1: comm_1,
        comm_2: comm_2,
    };

    let s = r + challenge * a1;
    let u = rho + challenge * r1;
    let t = tau + challenge * r2;
    let response = ProofResponse { s, u, t };
    let proof = Proof {
        commitment: commitment,
        response: response,
    };

    println!("compute done!");

    // verifying
    let (instance, witness, proof): (Instance, (Fr, Fr, Fr, Fr), Proof) = (
        // Your solution here!
        ins,
        (a1, r1, a2, r2),
        proof,
    );

    let (a_1, r_1, a_2, r_2) = witness;

    println!("a_1 = {}", a_1);
    println!("r_1 = {}", r_1);
    println!("a_2 = {}", a_2);
    println!("r_2 = {}", r_2);
    println!("");

    println!("C_rho = {}", proof.commitment.comm_rho);
    println!("C_tau = {}", proof.commitment.comm_tau);
    println!("");

    println!("s = {}", proof.response.s);
    println!("u = {}", proof.response.u);
    println!("t = {}", proof.response.t);

    assert!(verify(&ck, &instance, &proof));
    // Check that commitments are correct
    assert_eq!(
        ck.commit_with_explicit_randomness(a_1, r_1),
        instance.comm_1
    );
    assert_eq!(
        ck.commit_with_explicit_randomness(a_2, r_2),
        instance.comm_2
    );
    // Check that messages are unequal
    assert_ne!(a_1, a_2);
}
