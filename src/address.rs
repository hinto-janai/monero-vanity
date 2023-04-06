//---------------------------------------------------------------------------------------------------- Use
use regex::Regex;
use rand::Rng;
use monero::{
	Network,
	PrivateKey,
	KeyPair,
	Address,
};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::edwards::EdwardsPoint;
use curve25519_dalek::constants::ED25519_BASEPOINT_TABLE;
use std::sync::{
	Arc,
};
use std::sync::atomic::{
	AtomicBool,
	AtomicU64,
};
//--------------------------------------------------------------------------------------------------- Constants.
// Mainnet Monero Network.
const NETWORK_BYTE: u8 = 18;
// Mainnet Monero Network.
const NETWORK_ARRAY: &[u8] = &[18];
// How many `EdwardsPoint`'s to
// compress in batch in one go.
const BATCH_SIZE: usize = 10_000;
const BATCH_SIZE_U64: u64 = BATCH_SIZE as u64;

//---------------------------------------------------------------------------------------------------- Spawn worker threads.
#[inline(always)]
pub fn spawn_workers(
	threads: usize,
	to_main: &std::sync::mpsc::Sender::<(String, String, String)>,
	iter: &Arc<AtomicU64>,
	die: &Arc<AtomicBool>,
	regex: &Regex,
) {
	for _ in 0..threads {
		let to_main = to_main.clone();
		let iter    = iter.clone();
		let die     = die.clone();
		let regex   = regex.clone();

		std::thread::spawn(move || calculate(to_main, iter, die, regex));
	}
}

//---------------------------------------------------------------------------------------------------- Random P_Key.
fn rand_scalar() -> Scalar {
	// Random [u8; 64]
	let mut x = [0u8; 64];
	rand::thread_rng().fill(&mut x[..]);
	Scalar::from_bytes_mod_order_wide(&x)
}

fn rand_priv() -> PrivateKey {
	PrivateKey { scalar: rand_scalar() }
}

//---------------------------------------------------------------------------------------------------- Calculate the address.
#[inline(always)]
fn calculate(
	to_main: std::sync::mpsc::Sender::<(String, String, String)>,
	iter: Arc<AtomicU64>,
	die: Arc<AtomicBool>,
	regex: Regex,
) {
	// Seed.
	let seed = rand_scalar();

	// Base Point.
	let mut point = &seed * &ED25519_BASEPOINT_TABLE;

	// Offset.
	let offset = &Scalar::from(1_u8) * &ED25519_BASEPOINT_TABLE;

	// Thread local iteration count.
	let mut tries: u64 = 0;

	loop {
		// Batch compress the `EdwardsPoint`'s.
		let mut eds: Vec<EdwardsPoint> = Vec::with_capacity(BATCH_SIZE);
		for i in 0..BATCH_SIZE {
			eds.push(point);
			point += offset;
		}
		let y_points = EdwardsPoint::batch_compress_edwards(&mut eds);

		// Iterate over `CompressedEdwardsY` (public key)
		for y in y_points {
			// Calculate 1st `11` characters of Monero address.
			let mut bytes = [0_u8; 11];
			bytes[0] = NETWORK_BYTE;
			bytes[1..].copy_from_slice(&y.as_bytes()[0..10]);

			let addr = &crate::encode::encode_11(&bytes);

			// Check for regex match.
			// SAFETY:
			// The input is known UTF-8 compatible bytes.
			if regex.is_match(unsafe { &std::str::from_utf8_unchecked(&addr[..]) }) {
				// If found, signal to other threads.
				die.store(true, std::sync::atomic::Ordering::SeqCst);

				// Create Private Spend/View Keypair.
				let spend = PrivateKey { scalar: seed + Scalar::from(tries) };
				let view = rand_priv();
				let pair = KeyPair { view, spend };

				let address = Address::from_keypair(Network::Mainnet, &pair);

				// Send to `GUI`.
				to_main.send((address.to_string(), spend.to_string(), view.to_string()));

				// Exit.
				break
			}

			// Else, increment.
			tries += 1;
		}

		// Exit if `die` signal is set.
		if die.load(std::sync::atomic::Ordering::SeqCst) == true {
			break
		}

		// Increment `iteration`.
		iter.fetch_add(BATCH_SIZE_U64, std::sync::atomic::Ordering::SeqCst);
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
