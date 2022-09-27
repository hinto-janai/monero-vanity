// monero-vanity
//
// Copyright (c) 2022 hinto.janaiyo
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use rand::Rng;
use regex::Regex;
use std::io;
use std::io::Write;
use std::thread;
use std::process::exit;
use std::time::Instant;
use num_cpus;
use curve25519_dalek::scalar::Scalar;
use monero::{PublicKey, PrivateKey, Address, Network};

fn main() {
	// Interations or guesses
	let mut tries = 0;

	// Monero network = Mainnet
	let network = Network::Mainnet;

	// Detect core count
	let detected_cores: u32 = num_cpus::get().try_into().unwrap();

	// Ask user on how many cores to use
	let mut input = String::new();
	let cores: u32;
	print!("How many cores to use? [0-{}] (0 = all cores): ", detected_cores);
	io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed read line");
	// Handle input
	let input: u32 = input.trim().parse().expect("Input was not a number");
	if input > detected_cores {
		println!("Input [{}] is greater than detected cores [{}]", input, detected_cores);
		exit(1);
	} else if input == 0 {
		cores = detected_cores;
	} else {
		cores = input;
	}

	// Get pattern type (prefix|regex)
	println!("");
	println!("What type to look for? [prefix|regex]");
	println!("    - Prefix: starts from 3rd character, ASCII only: \"hinto\" = [48hinto...]");
	println!("    - Regex: starts from 1st character, Rust regex: \"^4[1-9]h(1|i)nto.*$\" = [44hinto...|42h1nto]");
	println!("    - There is no speed difference between these two.");
	print!("Type: ");
	io::stdout().flush().unwrap();
	let mut input = String::new();
	let pattern_type;
    io::stdin().read_line(&mut input).expect("Failed read line");
	if Regex::is_match(&Regex::new("reg").unwrap(), &input) {
		pattern_type = "regex";
	} else {
		pattern_type = "prefix";
	}

	println!("Using pattern type: [{}]", pattern_type);

	// Get address pattern
	println!("");
	if pattern_type == "regex" {
		println!("What pattern to look for?");
		println!("    - Must not include 'I', 'O', 'l' ");
		println!("    - Will start from the 1st character");
		println!("    - [48hinto...] would match if \"^48hinto.*$\" was typed");
		println!("    - [48hinto...|44h1nto...] would match if \"^4(4|8)h(i|1)nto.*$\" was typed");
		println!("    - Rust regex patterns are used: https://docs.rs/regex/latest/regex ");
		print!("Pattern: ");
	} else {
		println!("What pattern to look for?");
		println!("    - Must be ASCII and not include 'I', 'O', 'l' ");
		println!("    - Will always start from the 3rd character, [48...]");
		println!("    - [48hinto...] would match if \"hinto\" was typed");
		print!("Pattern: ");
	}
	io::stdout().flush().unwrap();
	let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed read line");
	// strip newline
	if input.ends_with('\n') {
		input.pop();
		if input.ends_with('\r') {
			input.pop();
		}
	}
	let pattern;
	if pattern_type == "regex" {
		pattern = format!("{}", input);
	} else {
		pattern = format!("^..{}.*$", input);
		if Regex::is_match(&Regex::new("(I|O|l)").unwrap(), &pattern) {
			println!("Pattern cannot contain 'I', 'O', or 'l'");
			exit(1);
		}
	}

	// Start
	println!("");
	print!("Using [{}] cores, looking for pattern [{}] with type [{}]... ", cores, Regex::new(&pattern).unwrap(), pattern_type);
	io::stdout().flush().unwrap();

	// Set timer
	let now = Instant::now();

	for _i in 1..=cores {
		let regex = Regex::new(&pattern).unwrap();
		thread::spawn(move|| loop {
			// Generate [32] random [u8] bytes to act as scalar
			let private = PrivateKey { scalar: Scalar::from_bytes_mod_order(rand::thread_rng().gen::<[u8; 32]>()), };
			let public = PublicKey::from_private_key(&private);
			if Regex::is_match(&regex, &Address::standard(network, public, public).to_string()) {
				println!("");
				println!("------------------------------------------------------------------------------------------------------------------");
				println!("Found in [{}] tries in [{:?}]!\n", tries * cores, now.elapsed());
			    println!("Private Spend Key | {}", private);
			    println!("Public Spend Key  | {}", public);
			    println!("Standard Address  | {}\n", Address::standard(network, public, public).to_string());
				println!("Recover with: monero-wallet-cli --generate-from-spend-key");
				println!("-------------------------------------------------------------------------------------------------------------------");
				exit(0);
			} else {
				tries += 1;
			}
		});
	}
	thread::park();
}
