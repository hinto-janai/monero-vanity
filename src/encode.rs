//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use benri::{
//};
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};

//---------------------------------------------------------------------------------------------------- Base58.
// This code is copied from `https://docs.rs/base58-monero`
// and optimized for performance. Since the input in `monero-vanity`
// is fixed-length and always correct, we can get rid of error handling,
// get rid of dynamic code, and optimize for the exact `11` characters
// the user wants matched.

//---------------------------------------------------------------------------------------------------- Constants
// Base58 alphabet, does not contains visualy similar characters
const BASE58_CHARS: [u8; 58] = *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BASE58_CHARS_LEN: u64 = BASE58_CHARS.len() as u64;

// The character chunks we're operating on.
// This is the max for Monero's `base58` and
// equate to around `8` bytes.
const CHUNK: usize = 11;

// Unrolled `u8be_to_u64()` function loop as a macro.
macro_rules! n {
    ($d:expr, $i:expr, $num:expr) => {
        $num = $num << 8 | $d[$i] as u64;
    }
}

// Unrolled `encode_block()` function loop as a macro.
macro_rules! r {
    ($i:expr, $num:expr, $res:expr) => {
        $res[$i] = BASE58_CHARS[($num % BASE58_CHARS_LEN) as usize];
        $num /= BASE58_CHARS_LEN;
    }
}

#[inline(always)]
/// INVARIANT:
/// Only uses the first `11` bytes of input.
pub fn encode_11(data: &[u8]) -> [u8; CHUNK] {
	let mut res = [0; CHUNK];
	let mut num = {
		let mut num: u64 = 0;
		n!(data, 0, num);
		n!(data, 1, num);
		n!(data, 2, num);
		n!(data, 3, num);
		n!(data, 4, num);
		n!(data, 5, num);
		n!(data, 6, num);
		n!(data, 7, num);
		num
	};
	r!(10, num, res);
	r!(9, num, res);
	r!(8, num, res);
	r!(7, num, res);
	r!(6, num, res);
	r!(5, num, res);
	r!(4, num, res);
	r!(3, num, res);
	r!(2, num, res);
	r!(1, num, res);
	r!(0, num, res);
	res
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	// This test checks _every_ 11-byte array
	// permutation and checks the base58 encoded
	// version is `11` in length.
	//
	// Forgive the ugliness.
	fn encode_all_11_byte_arrays() {
		for a in 0..=255 {
		for b in 0..=255 {
		for c in 0..=255 {
		for d in 0..=255 {
		for e in 0..=255 {
		for f in 0..=255 {
		for g in 0..=255 {
		for h in 0..=255 {
		for i in 0..=255 {
		for j in 0..=255 {
		for k in 0..=255 {
		    let array = [a, b, c, d, e, f, g, h, i, j, k];
		    let addr = encode_11(&array);
		    assert!(unsafe { std::str::from_utf8_unchecked(&addr) }.len() == 11);
		}}}}}}}}}}}
	}
}
