use bls_pedersen::bls::verify;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

use ark_bls12_381::G1Affine;
use std::io::Cursor;
use ark_serialize::CanonicalDeserialize;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (pk, ms, sigs) = puzzle_data();
    // for (m, sig) in ms.iter().zip(sigs.iter()) {
    //     verify(pk, m, *sig);
    // }

    /* Your solution here! */
	let username = "obeliskgolem";
	let username = username.to_string();
	let s = "10d3ec2496aaf0a9f1288ad2cc973b37dc5c300ca7206f977fabf5136c16b0356789c98613aad38b92f5788af3a88313";
	let sig = G1Affine::deserialize(&mut Cursor::new(hex::decode(s).unwrap())).unwrap();
	verify(pk, username.as_bytes(), sig);
}
