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
const NETWORK: &[u8] = &[18];

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
		// Process in `10_000` iteration batches.
		for _ in 0..10_000 {
			// Calculate 1st half of Monero address.
			let addr = &base58_monero::encode(&[NETWORK, point.compress().as_bytes()].concat()).unwrap()[..=43];

			// Check for regex match.
			if regex.is_match(&addr) {
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
			point += offset;
		}

		// Exit if `die` signal is set.
		if die.load(std::sync::atomic::Ordering::SeqCst) == true {
			break
		}

		// Increment `iteration`.
		iter.fetch_add(10_000, std::sync::atomic::Ordering::SeqCst);
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
