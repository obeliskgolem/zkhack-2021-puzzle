#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use ark_bls12_381::{G1Affine, G1Projective, Fr};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{One, Zero};
use ark_std::ops::Sub;
use prompt::{puzzle, welcome};
use soundness_of_music::{data::puzzle_data, prover, prover::Proof, verifier, PUZZLE_DESCRIPTION};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (circuit, setup) = puzzle_data();

    let public_inputs = [Fr::one(), Fr::one()];

    // /* Your solution here! */
    // let one = Fr::one();
    // let two = one + one;
    // let four = two + two;
    // let valid_public_inputs = [one, four];
    // let valid_proof = prover::prove(&valid_public_inputs, &[two], &circuit, &setup);
 
    // let (a, b) = valid_public_inputs
    //     .iter()
    //     .zip(public_inputs.iter())
    //     .zip(setup.inputs.iter().zip(setup.inputs_prime.iter()))
    //     .fold(
    //         (G1Projective::zero(), G1Projective::zero()),
    //         |(mut acc, mut acc_prime), ((valid_pub_in, pub_in), (setup_in, setup_in_prime))| {
    //             acc += setup_in.mul(valid_pub_in.sub(pub_in));
    //             acc_prime += setup_in_prime.mul(valid_pub_in.sub(pub_in));
    //             (acc, acc_prime)
    //         },
    //     );
    // let (a, b) = (a.into_affine(), b.into_affine());

    // let forge_pi_input = valid_proof.pi_input + a;
    // let forge_pi_input_prime = valid_proof.pi_input_prime + b;

    // let forge_proof = Proof {
    //     pi_input: forge_pi_input,
    //     pi_input_prime: forge_pi_input_prime,
    //     pi_output: valid_proof.pi_output,
    //     pi_output_prime: valid_proof.pi_output_prime,
    //     pi_K: valid_proof.pi_K,
    //     pi_H: valid_proof.pi_H,
    // };


    /* Your solution here! */
    let public_inputs = [Fr::one(), Fr::one()];
    let fake_proof = prover::prove_alter(&public_inputs, 1, &setup);

    // Replace unimplmented!() with your proof
    // assert!(verifier::verify(&public_inputs, &setup, &forge_proof));
    assert!(verifier::verify(&public_inputs, &setup, &fake_proof));
}
