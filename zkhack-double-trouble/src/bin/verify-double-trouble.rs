#![allow(unused, unreachable_code)]
use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use double_trouble::data::puzzle_data;
use double_trouble::inner_product_argument::utils::challenge;
use double_trouble::verify;
use double_trouble::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ed_on_bls12_381::EdwardsAffine as GAffine;
use ark_ff::Zero;
use ark_serialize::CanonicalDeserialize;
use ark_serialize::CanonicalSerialize;
use double_trouble::ProofCommitment;
use std::io::Cursor;
use std::str::FromStr;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (ck, [instance_and_proof_1, instance_and_proof_2]) = puzzle_data();

    println!("ck.generators.len = {}", ck.generators.len());
    let (instance1, proof1) = instance_and_proof_1;
    let (instance2, proof2) = instance_and_proof_2;

    assert!(verify(&ck, &instance1, &proof1));
    assert!(verify(&ck, &instance2, &proof2));

    // println!("instance 1.comm_a = {}", instance1.comm_a);
    // for i in 0..instance1.b.len() {
    //     println!("instance1.b[{}] = {}", i, instance1.b[i]);
    // }

    // println!("");
    // println!("proof1.commitment.comm_r = {:?}", proof1.commitment.comm_r);
    // println!("proof1.commitment.comm_1 = {:?}", proof1.commitment.comm_1);
    // println!("proof1.commitment.comm_2 = {:?}", proof1.commitment.comm_2);

    // println!("");
    // println!("proof1.response.u = {:?}", proof1.response.u);
    // println!("proof1.response.t = {:?}", proof1.response.t);
    // for i in 0..proof1.response.s.len() {
    //     println!("proof1.response.s[{}] = {}", i, proof1.response.s[i]);
    // }

    // println!("");
    // println!("================");
    // println!("");

    // println!("instance 2.comm_a = {}", instance2.comm_a);
    // for i in 0..instance2.b.len() {
    //     println!("instance2.b[{}] = {}", i, instance2.b[i]);
    // }

    // println!("");
    // println!("proof2.commitment.comm_r = {:?}", proof2.commitment.comm_r);
    // println!("proof2.commitment.comm_1 = {:?}", proof2.commitment.comm_1);
    // println!("proof2.commitment.comm_2 = {:?}", proof2.commitment.comm_2);

    // println!("");
    // println!("proof2.response.u = {:?}", proof2.response.u);
    // println!("proof2.response.t = {:?}", proof2.response.t);
    // for i in 0..proof2.response.s.len() {
    //     println!("proof2.response.s[{}] = {}", i, proof2.response.s[i]);
    // }

    let c1 = challenge(&ck, &instance1, &proof1.commitment);
    let c2 = challenge(&ck, &instance2, &proof2.commitment);
    let u1 = proof1.response.u;
    let u2 = proof2.response.u;
    let n_2 = Fr::from_str("2").unwrap();

    let rho = (u2 - u1) / (n_2 * c2 - c1);
    let alpha = u1 - c1 * rho;

    // println!("rho = {}", rho);
    println!("alpha = {}", alpha);

    let mut r_v = Vec::new();
    for i in 0..proof1.response.s.len() {
        let s = proof1.response.s[i] - proof2.response.s[i];
        let c = c1 - n_2 * c2;
        r_v.push(s / c);
        // println!("r[{}] = {}", i, s / c);
    }

    let mut a_v = Vec::new();
    for i in 0..proof1.response.s.len() {
        let a = proof1.response.s[i] - (c1 * r_v[i]);
        a_v.push(a);
        println!("a[{}] = {}", i, a);
    }

    // Your solution here!
    // alpha = Fp256 "(039356E0074288047BE3CD30583A88B7A760D6AFC2C1E996C33A41684D989534)"
    // a[0] = Fp256 "(052A875D7E3F3F47D964D8D03C9C96BAC72690BD62D529233A5F196EA626DA47)"
    // a[1] = Fp256 "(0709F70E0787B6F99008CAD06522A40A7C4A0B4D62E8029B7073F64D6832E97C)"
    // a[2] = Fp256 "(0B72FDA7B32B74DF0EE544558104AE1B79318A4BF2F768F045140EA48EE8CC2E)"
    // a[3] = Fp256 "(04CFCDCCAF8D3DA832B5F677B3D20B27444C77AB52901ABA1C4F4A124C6C8BFA)"
    // a[4] = Fp256 "(0BBDE6DD34E9A0C21118ACAB8083F6308F297DB567D3AC7DF3BE555335FECC85)"
    // a[5] = Fp256 "(0CD666318404B4C1042C3B1646E32F072A10C30F05FAA2701A3F25C987470DB5)"
    // a[6] = Fp256 "(023961D1A8FCD42B7BB8EEE8CBCBE46A1FAEBB65320BABF09A635FDF8C9821F9)"
    // a[7] = Fp256 "(03A52AF29CA3C162D31CD9D8E8937B652E0BCEC0E3C03270E3F82C91920C2A56)"

    let (a, comm_a_rand): (Vec<Fr>, Fr) = { (a_v, alpha) };

    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance1.comm_a
    );
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance2.comm_a
    );
}
