#![allow(unused, unreachable_code, dead_code)]

use ark_bls12_381::{Fr, G1Affine};
use ark_ff::*;
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, GeneralEvaluationDomain, Polynomial,
    UVPolynomial,
};
use ark_serialize::CanonicalDeserialize;
use hidden_in_plain_sight::{generate::kzg_commit, PUZZLE_DESCRIPTION};
use prompt::{puzzle, welcome};

use ark_poly::univariate::DenseOrSparsePolynomial;
use hidden_in_plain_sight::generate::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::Read;

fn read_cha_from_file() -> (Vec<G1Affine>, Vec<Vec<Fr>>, Fr, Fr, G1Affine, Fr, Fr) {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("challenge_data").unwrap();
    let mut bytes: Vec<u8> = vec![];
    file.read_to_end(&mut bytes).unwrap();

    let setup_bytes: Vec<u8> = bytes[0..98312].to_vec();
    let accts_bytes: Vec<u8> = bytes[98312..1130320].to_vec();
    let cha_1_bytes: Vec<u8> = bytes[1130320..1130352].to_vec();
    let cha_2_bytes: Vec<u8> = bytes[1130352..1130384].to_vec();
    let commt_bytes: Vec<u8> = bytes[1130384..1130480].to_vec();
    let opn_1_bytes: Vec<u8> = bytes[1130480..1130512].to_vec();
    let opn_2_bytes: Vec<u8> = bytes[1130512..1130544].to_vec();

    let setup = Vec::<G1Affine>::deserialize_unchecked(&setup_bytes[..]).unwrap();
    let accts = Vec::<Vec<Fr>>::deserialize_unchecked(&accts_bytes[..]).unwrap();
    let cha_1 = Fr::deserialize_unchecked(&cha_1_bytes[..]).unwrap();
    let cha_2 = Fr::deserialize_unchecked(&cha_2_bytes[..]).unwrap();
    let commt = G1Affine::deserialize_unchecked(&commt_bytes[..]).unwrap();
    let opn_1 = Fr::deserialize_unchecked(&opn_1_bytes[..]).unwrap();
    let opn_2 = Fr::deserialize_unchecked(&opn_2_bytes[..]).unwrap();

    (setup, accts, cha_1, cha_2, commt, opn_1, opn_2)
}

fn calculate_commitment(
    setup: &Vec<G1Affine>,
    p: Vec<Fr>,
    c1: Fr,
    c2: Fr,
    q_c1: Fr,
    q_c2: Fr,
    check: G1Affine,
    i: usize,
) -> Option<Vec<Fr>> {
    let n = setup.len();
    let domain: GeneralEvaluationDomain<Fr> = GeneralEvaluationDomain::new(n).unwrap();
    let poly = DensePolynomial::from_coefficients_vec(domain.ifft(&p));

    let p_c1 = poly.evaluate(&c1);
    let p_c2 = poly.evaluate(&c2);

    let c1_n_minus_one = c1.pow(&[n as u64]) - Fr::one();
    let c2_n_minus_one = c2.pow(&[n as u64]) - Fr::one();

    let b0 = ((q_c1 - p_c1) * (c2_n_minus_one) * c2 - (q_c2 - p_c2) * (c1_n_minus_one) * c1)
        / ((c1_n_minus_one) * (c2_n_minus_one) * (c2 - c1));
    let b1_1 = ((q_c1 - p_c1) - (c1_n_minus_one * b0)) / (c1_n_minus_one * c1);
    let b1_2 = ((q_c2 - p_c2) - (c2_n_minus_one * b0)) / (c2_n_minus_one * c2);

    assert_eq!(b1_1, b1_2);

    let blinding_poly = DensePolynomial::from_coefficients_vec(vec![b0, b1_1]);

    let checking_poly = poly + blinding_poly.mul_by_vanishing_poly(domain);

    let commitment: G1Affine = kzg_commit(&checking_poly, &setup);

    if commitment == check {
        println!("commitment founded at index {}", i);
        Some(checking_poly.to_vec())
    } else {
        None
    }
}

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);

    let (setup, accts, cha_1, cha_2, commt, opn_1, opn_2) = read_cha_from_file();

    // let (setup, accts, cha_1, cha_2, commt, opn_1, opn_2) = generate_challenge();

    let mut file = File::open("challenge_data").unwrap();
    let mut bytes: Vec<u8> = vec![];
    file.read_to_end(&mut bytes).unwrap();

    let accts_bytes: Vec<u8> = bytes[98312..1130320].to_vec();

    println!("setup.len = {}", setup.len());

    // let acc_len = accts.len();
    // let mut acc_check = vec![false; accts.len()];
    // accts.clone()
    //     .into_par_iter()
    //     .zip((0..acc_len).into_par_iter())
    //     .map(|(acct, i)| {
    //         println!("checking accts[{}]: ", i);
    //         calculate_commitment(&setup, acct.clone(), cha_1, cha_2, opn_1, opn_2, commt, i)
    //     })
    //     .collect::<Vec<_>>();

    // found i = 535
    let index = 535;
    let mut checking_poly_vec = calculate_commitment(
        &setup,
        accts[index].clone(),
        cha_1,
        cha_2,
        opn_1,
        opn_2,
        commt,
        index,
    )
    .unwrap();

    checking_poly_vec.truncate(setup.len());

    println!("====  poly  ====");
    for i in 0..checking_poly_vec.len() {
        println!("acct_poly[{}] = {}", i, checking_poly_vec[i]);
    }

    println!("====  address  ====");
    for i in 0..accts[index].len() {
        println!("{}", accts[index][i].into_repr());
    }

    // address = "0xDA87BE72D77766E5B65AA9F3CE0C5664A49246734EDDC150AA0D0871ADBC05C1"

    // Replace with the solution polynomial, derived from the account!
    let solution_blinded_acct = DensePolynomial::from_coefficients_vec(checking_poly_vec);

    let solution_commitment = kzg_commit(&solution_blinded_acct, &setup);
    assert_eq!(solution_commitment, commt);
}
