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
// and edited for performance. Since the input in `monero-vanity`
// is known and always correct, we can get rid of some `Result`'s and `.unwrap()`'s.

//---------------------------------------------------------------------------------------------------- Constants
/// Base58 alphabet, does not contains visualy similar characters
pub const BASE58_CHARS: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
/// Resulted block size given a `0..=8` bytes block
pub const ENCODED_BLOCK_SIZES: [usize; 9] = [0, 2, 3, 5, 6, 7, 9, 10, 11];
/// Maximum size of block to encode
pub const FULL_BLOCK_SIZE: usize = 8;
/// Size of an encoded 8 bytes block, i.e. maximum encoded block size
pub const FULL_ENCODED_BLOCK_SIZE: usize = ENCODED_BLOCK_SIZES[FULL_BLOCK_SIZE];
/// Size of checksum
pub const CHECKSUM_SIZE: usize = 4;

//---------------------------------------------------------------------------------------------------- Encode
fn u8be_to_u64(data: &[u8]) -> u64 {
	let mut res = 0u64;
	for b in data {
		res = res << 8 | *b as u64;
	}
	res
}

fn encode_block(data: &[u8]) -> Result<[char; FULL_ENCODED_BLOCK_SIZE]> {
	if data.is_empty() || data.len() > FULL_BLOCK_SIZE {
		return Err(Error::InvalidBlockSize);
	}
	let mut res = ['1'; FULL_ENCODED_BLOCK_SIZE];
	let mut num = u8be_to_u64(data);
	let mut i = ENCODED_BLOCK_SIZES[data.len()];
	while i > 0 {
		let remainder: usize = (num % BASE58_CHARS.len() as u64) as usize;
		num /= BASE58_CHARS.len() as u64;
		i -= 1;
		res[i] = BASE58_CHARS[remainder] as char;
	}
	Ok(res)
}

/// Encode a byte vector into a base58-encoded string
pub fn encode(data: &[u8]) -> Result<String> {
    let last_block_size = ENCODED_BLOCK_SIZES[data.len() % FULL_BLOCK_SIZE];
    let full_block_count = data.len() / FULL_BLOCK_SIZE;
    let data: Result<Vec<[char; FULL_ENCODED_BLOCK_SIZE]>> =
        data.chunks(FULL_BLOCK_SIZE).map(encode_block).collect();

    let mut i = 0;
    let mut res: Vec<char> = Vec::new();
    data?.into_iter().for_each(|v| {
        if i == full_block_count {
            res.extend_from_slice(&v[..last_block_size]);
        } else {
            res.extend_from_slice(&v);
        }
        i += 1;
    });

    let s: String = res.into_iter().collect();
    Ok(s)
}


//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
