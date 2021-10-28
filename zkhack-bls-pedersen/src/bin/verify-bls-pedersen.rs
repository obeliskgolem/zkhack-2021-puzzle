use bls_pedersen::bls::verify;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

use ark_bls12_381::{Fr};
use ark_crypto_primitives::crh::pedersen::bytes_to_bits;
use ark_ec::{msm::VariableBaseMSM, ProjectiveCurve};
use ark_ff::{fields::PrimeField, One, Zero};
use ark_serialize::CanonicalSerialize;

fn get_bits_u8(x: &[u8], i: usize) -> bool {
    let bits = bytes_to_bits(x);

    bits[i]
}

fn gauss(hm_table: &mut [Vec<u8>], target: &[u8], result: &mut [Fr]) {
    let n = 256;
    let mut f = vec![vec![Fr::zero(); n]; n];
    let mut b_origin = vec![Fr::zero(); n];

    let mut a = vec![vec![Fr::zero(); n]; n];
    let mut b = vec![Fr::zero(); n];
    let mut r = Vec::new();

    for i in 0..n {
        r.push(result[i]);
    }

    // init f
    for i in 0..n {
        for j in 0..n {
            if get_bits_u8(&hm_table[i][..], j) {
                f[j][i] = Fr::one();
                a[j][i] = Fr::one();
            } else {
                f[j][i] = Fr::zero();
                a[j][i] = Fr::zero();
            }
        }
    }

    // init b
    for i in 0..n {
        if get_bits_u8(target, i) {
            b[i] = Fr::one();
            b_origin[i] = Fr::one();
        } else {
            b[i] = Fr::zero();
            b_origin[i] = Fr::zero();
        }
    }

    // re-ordering so that a[i][i] != 0
    for i in 0..n {
        if a[i][i] == Fr::zero() {
            // if !get_bits_u8(&hm_table[i][..], i) {
            let mut found = false;
            for j in 0..n {
                if i != j && a[i][j] == Fr::one() && a[j][i] == Fr::one() {
                    let s = a[i].clone();
                    a[i] = a[j].clone();
                    a[j] = s.clone();

                    let o = b[j].clone();
                    b[j] = b[i].clone();
                    b[i] = o.clone();
                    found = true;

                    break;
                }
            }

            if !found {
                panic!("error, a[{}][{}]=0", i, i);
            }
        }
    }

    // forward
    for i in 0..n - 1 {
        if a[i][i] == Fr::zero() {
            let mut found = false;
            for k in i + 1..n {
                if a[k][i] != Fr::zero() && a[i][k] != Fr::zero() {
                    let s = a[k].clone();
                    a[k] = a[i].clone();
                    a[i] = s.clone();

                    let o = b[k].clone();
                    b[k] = b[i].clone();
                    b[i] = o.clone();
                    found = true;

                    break;
                }
            }
            if !found {
                panic!("error, pivot zero");
            }
        }

        for k in i + 1..n {
            let s = -(a[k][i] / a[i][i]);
            // println!("in forward, s = {}", s);

            for j in i..n {
                a[k][j] = a[i][j] * s + a[k][j];
            }

            b[k] = b[i] * s + b[k];
        }
    }

    // backward
    for i in (0..n).rev() {
        let mut s = b[i];
        for j in i + 1..n {
            s = s - a[i][j] * r[j];
        }

        r[i] = s / a[i][i];
    }

    for i in 0..n {
        result[i] = r[i].clone();
    }

    for i in 0..n {
        let mut s = Fr::zero();
        for j in 0..n {
            s = s + f[i][j] * r[j];
        }
        if s != b_origin[i] {
            panic!("s = {}, b_origin[{}] = {}", s, i, b_origin[i]);
        }
    }
}

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (pk, ms, sigs) = puzzle_data();
    // for (m, sig) in ms.iter().zip(sigs.iter()) {
    //     verify(pk, m, *sig);
    // }

    let username = "obeliskgolem".to_string();
    let b2hash = blake2s_simd::blake2s(username.as_bytes());
    let target = b2hash.as_bytes().clone();
    println!("username = {}, b2hash = {:?}", username, b2hash);

    let mut hm_table = Vec::new();
    let mut result = vec![Fr::zero(); 256];

    for i in 0..ms.len() {
        let b2hash = blake2s_simd::blake2s(&ms[i]);
        let s = b2hash.as_bytes().to_vec();
        hm_table.push(s.clone());
    }

    gauss(&mut hm_table[..], &target[..], &mut result[..]);

    let mut r_bigint = Vec::new();

    for i in 0..result.len() {
        r_bigint.push(result[i].into_repr());
    }

    let g = VariableBaseMSM::multi_scalar_mul(&sigs[..], &r_bigint[..]);

    println!("g = {}", g);

    // Your solution here!
    // sig == "10d3ec2496aaf0a9f1288ad2cc973b37dc5c30ca7206f977fabf5136c16b0356789c98613aad38b92f5788af3a88313"
    let sig = g.into_affine();

    let mut w = Vec::new();
    sig.serialize(&mut w).unwrap();

    print!("sig serialization = ");
    for i in 0..w.len() {
        print!("{:x}", w[i]);
    }
    println!("");

    let m = username.as_bytes();
    verify(pk, &m, sig);
}
