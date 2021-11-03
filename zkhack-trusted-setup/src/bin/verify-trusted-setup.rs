use ark_bls12_381::{Fq, Fr};
use ark_ec::AffineCurve;
use prompt::{puzzle, welcome};
use std::str::FromStr;
use trusted_setup::data::puzzle_data;
use trusted_setup::PUZZLE_DESCRIPTION;

use ark_ec::ProjectiveCurve;
use ark_ff::fields::SquareRootField;
use ark_bls12_381::{G1Affine, G2Affine};
use ark_ff::Field;
use ark_ff::Zero;

fn get_order_g1(p: G1Affine) {
    let r = Fr::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184512",
    )
    .unwrap();

    let a0 = Fr::from_str("3").unwrap();
    let a1 = Fr::from_str("11").unwrap();
    let a2 = Fr::from_str("10177").unwrap();
    let a3 = Fr::from_str("859267").unwrap();
    let a4 = Fr::from_str("52437899").unwrap();

    for i0 in 0..=1 {
        for i1 in 0..=2 {
            for i2 in 0..=2 {
                for i3 in 0..=2 {
                    for i4 in 0..=2 {
                        for ir in 0..=1 {
                            let mut g1 = p.clone();

                            let a0 = a0.pow([i0]);
                            let a1 = a1.pow([i1]);
                            let a2 = a2.pow([i2]);
                            let a3 = a3.pow([i3]);
                            let a4 = a4.pow([i4]);

                            // println!("a0 = {}, a1 = {}, a2 = {}, a3 = {}, a4 = {}", a0, a1, a2, a3, a4);

                            if ir == 1 {
                                g1 = g1.mul(r).into_affine() + g1;
                            }

                            let mut g1 = g1.mul(a0 * a1 * a2 * a3 * a4);
                            if g1 == G1Affine::zero() {
                                println!("found G1 zero point at {} {} {} {} {} {}", i0, i1, i2, i3, i4, ir);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    panic!("G1 order not found!");
}


fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (_ts1, _ts2) = puzzle_data();

    /* Your solution here! (s in decimal)*/
    let s = Fr::from_str("114939083266787167213538091034071020048").unwrap();

    assert_eq!(_ts1[0].mul(s), _ts1[1]);
    assert_eq!(_ts2[0].mul(s), _ts2[1]);
}
