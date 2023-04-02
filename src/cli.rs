//---------------------------------------------------------------------------------------------------- Use
use clap::Parser;
use std::process::exit;
use crate::constants::{
	VERSION,
	COMMIT,
};
use crate::threads::{
	THREADS_HALF,
	THREADS_MAX,
};
use crate::state::State;
use regex::Regex;
use std::io::Write;
use readable::Int;

//---------------------------------------------------------------------------------------------------- CLI
const ABOUT: &str =
r#"monero-vanity automatically prefixes your input
with `^4.` and suffixes it with `.*$` so that
your PATTERN starts from the 3rd character
until the 43rd character of the address.

Example input: `hinto`
Actual regex used: `^4.hinto.*$`

To disable this, use `--first`.
Warning: this puts you in full control of the regex,
you can input any value, even an impossible one."#;

#[derive(Parser, Debug)]
#[command(version, override_usage = "monero-vanity [--OPTIONS]", long_about = ABOUT)]
pub struct Cli {
	/// How many threads to use.
	///
	/// Will use half threads if no number or 0 is given.
	#[arg(long, short, default_value_t = *THREADS_HALF)]
	threads: usize,

	/// Address regex pattern to look for
	///
	/// E.g: `hinto` would find an address: `44hinto...`
	#[arg(long, short)]
	pattern: String,

	/// Start from 1st character instead of: ^4.PATTERN.*$
	#[arg(long, short)]
	first: bool,

	/// How many milliseconds in-between output refreshes
	#[arg(long, short, default_value_t = 500)]
	refresh: u64,
}

impl Cli {
	//-------------------------------------------------- CLI argument handling
	#[inline(always)]
	pub fn handle_args() {
		let cli = Self::parse();

		// Version.
		if cli.version {
			println!("monero-vanity v{} | Commit: {}", VERSION, COMMIT);
			exit(0);
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
			false => format!("^4.{}.*$", cli.pattern),
		};
		let pattern = match Regex::new(&pattern_string) {
			Ok(p) => p,
			Err(e) => { eprintln!("ERROR: Regex failed to build: {}", e); exit(8); },
		};

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
			..Default::default()
		};

		// Continue to loop.
		Self::cli_loop(state, cli.refresh);
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
				println!("\n\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
				println!("Monero Address    | {}", m.0);
				println!("Private Spend Key | {}", m.1);
				println!("Private View Key  | {}", m.2);
				println!("Tries             | {}", Int::from(iter));
				println!("Speed             | {} keys per second\n", Int::from(crate::speed::calculate(&state.start, iter)));
				println!("Recover with: ./monero-wallet-cli --generate-from-spend-key YOUR_WALLET_NAME");
				println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
				std::process::exit(0);
			}

			print!(
				"\rTries: [{}] | Speed: [{} keys per second]",
				Int::from(iter),
				Int::from(crate::speed::calculate(&state.start, iter)),
			);
			std::io::stdout().lock().flush();

			std::thread::sleep(std::time::Duration::from_millis(refresh));
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
