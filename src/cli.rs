//---------------------------------------------------------------------------------------------------- Use
use clap::Parser;
use std::process::exit;
use std::str::FromStr;
use crate::constants::{
	VERSION,
	COMMIT,
	VERSION_COMMIT,
};
use crate::threads::{
	THREADS_HALF,
	THREADS_MAX,
};
use crate::state::State;
use regex::Regex;
use std::io::Write;
use readable::{
	Unsigned,
	Time,
};

//---------------------------------------------------------------------------------------------------- CLI
const ABOUT: &str =
r#"monero-vanity automatically prefixes your input
with `^..` and suffixes it with `.*$` so that
your PATTERN starts from the 3rd character
until the 11th character of the address.

Example input: `hinto`
Actual regex used: `^..hinto.*$`

To disable this, use `--first`.
Warning: this puts you in full control of the regex,
you can input any value, even an impossible one."#;

#[derive(Parser, Debug)]
#[command(version = VERSION_COMMIT, override_usage = "monero-vanity [--OPTIONS]", long_about = ABOUT)]
pub struct Cli {
	/// How many threads to use.
	///
	/// Will use half threads if no number or 0 is given.
	#[arg(long, short, default_value_t = *THREADS_HALF)]
	threads: usize,

	/// Address regex pattern to look for
	///
	/// E.g: `hinto` would find an address: `44hinto...`
	#[arg(long, short, default_value_t = String::new())]
	pattern: String,

	/// Start from 1st character instead of: ^..PATTERN.*$
	#[arg(long, short)]
	first: bool,

	/// How many milliseconds in-between output refreshes
	#[arg(long, short, default_value_t = 500)]
	refresh: u64,

	/// Generates a new split key that can be given out to allow 
	/// others to help you find an address while keeping the private 
	/// key hidden
	/// (experimental)
	#[arg(long, short)]
	gen_private_split_key: bool,

	/// Calculates addresses for the provided public split key
	/// instead of our own generated private key
	/// (experimental)
	#[arg(long, short)]
	calculate_split_key: Option<String>,

	/// Joins the private part of a split key with the calculated part
	/// to get the generated private key  
	/// (experimental)
	#[arg(long, short, num_args(2))]
	join_split_key: Option<Vec<String>>
}

impl Cli {
	//-------------------------------------------------- CLI argument handling
	#[inline(always)]
	pub fn handle_args() {
		let cli = Self::parse();
		if cli.gen_private_split_key {
			Self::gen_private_split_key()
		}
		if let Some(keys) = cli.join_split_key {
			Self::join_split_key(keys)
		}

		// Test for `pattern` validity.
		if cli.pattern.is_empty() {
			eprintln!("ERROR: Address pattern is empty");
			exit(1);
		} else if cli.pattern.contains('I') {
			eprintln!("ERROR: Address pattern must not contain 'I'");
			exit(2);
		} else if cli.pattern.contains('O') {
			eprintln!("ERROR: Address pattern must not contain 'O'");
			exit(3);
		} else if cli.pattern.contains('l') {
			eprintln!("ERROR: Address pattern must not contain 'l'");
			exit(4);
		} else if cli.pattern.contains('0') {
			eprintln!("ERROR: Address pattern must not contain '0'");
			exit(5);
		} else if cli.pattern.contains('+') {
			eprintln!("ERROR: Address pattern must not contain '+'");
			exit(6);
		} else if cli.pattern.contains('/') {
			eprintln!("ERROR: Address pattern must not contain '/'");
			exit(7);
		}
		let pattern_string = match cli.first {
			true  => cli.pattern,
			false => format!("^..{}.*$", cli.pattern),
		};
		let pattern = match Regex::new(&pattern_string) {
			Ok(p) => p,
			Err(e) => { eprintln!("ERROR: Regex failed to build: {}", e); exit(8); },
		};

		let split_key = cli.calculate_split_key.map(
			|key| match monero::PublicKey::from_str(&key) {
				Ok(key) => key.point.decompress().expect("monero-rs decompresses public keys so all `PublicKey`s will be valid points"),
				Err(e) => {eprintln!("ERROR: Public key entered is not a valid point: {}", e); exit(9); }
			}
		);

		// Test for `thread` validity.
		let threads = {
			// Use half if `0`.
			if cli.threads == 0 {
				eprintln!("[0] threads selected, defaulting to 50% of available threads: [{}]", *THREADS_HALF);
				*THREADS_HALF
			// Use max if over.
			} else if cli.threads > *THREADS_MAX {
				eprintln!(
					"[{}] threads selected, but only [{}] threads detected. Using [{}] threads.",
					cli.threads,
					*THREADS_MAX,
					*THREADS_MAX
				);
				*THREADS_MAX
			// Else, use user input.
			} else {
				cli.threads
			}
		};

		// Make `State`.
		let state = State {
			threads,
			pattern,
			pattern_string,
			split_key,
			..Default::default()
		};

		// Continue to loop.
		Self::cli_loop(state, cli.refresh);
	}

	fn gen_private_split_key() {
		let (private_part, public_part) = crate::address::calculate_part_split_key();
		let mut output = String::new();
		output += &format!("Private Split Key (keep hidden)   | {}\n", private_part);
		output += &format!("Public Split Key (give this out)  | {}\n", public_part);
		output += &format!("generate other part with: ./monero-vanity --calculate-split-key {} --pattern PATTERN_YOU_WANT", public_part);
		successful_exit(&output)
	}

	fn join_split_key(keys: Vec<String>) {
		let keys: Vec<monero::PrivateKey> = keys.iter().map(|key| match monero::PrivateKey::from_str(&key) {
			Ok(key) => key,
			Err(e) => {eprintln!("ERROR: Private key part entered is not a valid scalar: {}", e); exit(10); }
		}).collect();
		let m = crate::address::join_split_key(keys[0], keys[1]);

		let mut output = String::new();
		output += &format!("Monero Address             | {}\n", m.0);
		output += &format!("Private Spend Key          | {}\n", m.1);
		output += &format!("Private View Key           | {}\n", m.2);
		output += &format!("Recover with: ./monero-wallet-cli --generate-from-spend-key YOUR_WALLET_NAME");
		successful_exit(&output)

	}

	//-------------------------------------------------- CLI loop.
	fn cli_loop(mut state: State, refresh: u64) {
		// Create channels to/from workers.
		let (to, from) = std::sync::mpsc::channel::<(String, String, String)>();

		// Set timer.
		state.start = std::time::Instant::now();

		// Spawn workers.
		crate::address::spawn_workers(
			state.threads,
			&to,
			&state.iter,
			&state.die,
			&state.pattern,
			state.split_key,
		);

		println!(
			"Threads | {}\nRefresh | {}ms\nPattern | {}\n",
			state.threads,
			refresh,
			state.pattern_string,
		);

		// Loop, printing stats and checking for msg every 1 second.
		loop {
			let iter = state.iter.load(std::sync::atomic::Ordering::SeqCst);

			if let Ok(m) = from.try_recv() {
				let mut output = String::new();
				output +=     &format!("Tries                      | {} \n", Unsigned::from(iter));
				output +=     &format!("Speed                      | {} keys per second", Unsigned::from(crate::speed::calculate(&state.start, iter)));
				output +=     &format!("Elapsed                    | {}\n", Time::from(&state.start.elapsed()));
				if state.split_key.is_some() {
					output += &format!("Calculated Split Key part  | {}\n", m.1);
					output += &format!("Join keys with: ./monero-vanity --join-split-key {} PRIVATE_SPLIT_KEY_PART",m.1);
				} else {
					output += &format!("Monero Address             | {}\n", m.0);
					output += &format!("Private Spend Key          | {}\n", m.1);
					output += &format!("Private View Key           | {}\n", m.2);
					output += &format!("Recover with: ./monero-wallet-cli --generate-from-spend-key YOUR_WALLET_NAME");
				}
				successful_exit(&output)
			}

			print!(
				"{}[2K\rTries: [{}] | Speed: [{} keys per second] | Elapsed: [{}]",
				27 as char,
				Unsigned::from(iter),
				Unsigned::from(crate::speed::calculate(&state.start, iter)),
				Time::from(&state.start.elapsed()),
			);
			std::io::stdout().lock().flush();

			std::thread::sleep(std::time::Duration::from_millis(refresh));
		}
	}
}

fn successful_exit(output: &str) {
	println!("\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
	println!("{}",output);
	println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
	std::process::exit(0);
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
