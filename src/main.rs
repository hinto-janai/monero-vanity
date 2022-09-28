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

use num_cpus;
use std::io;
use std::io::Write;
use std::thread;
use std::process::exit;
use std::time::Instant;
use std::ops::AddAssign;
use regex::Regex;
use rand::rngs::OsRng;
use curve25519_dalek::scalar::Scalar;
use monero::{PublicKey, PrivateKey, Address, Network};

fn main() {
	// Ask user for core count
	let detected_cores: u32 = num_cpus::get().try_into().unwrap();
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
	println!("What type to look for? [first|third|full]");
	println!("    - Third: matches 3rd-43rd character");
	println!("    - First: matches 1st-43rd character");
	println!("    - Full: matches 1st-95th character");
	print!("Type: ");
	io::stdout().flush().unwrap();
	let mut input = String::new();
	let pattern_type;
    io::stdin().read_line(&mut input).expect("Failed read line");
	// Strip newline off input
	if input.ends_with('\n') {
		input.pop();
		if input.ends_with('\r') {
			input.pop();
		}
	}
	if Regex::is_match(&Regex::new("^(F|f)(I|i).*$").unwrap(), &input) {
		pattern_type = "first";
	} else if Regex::is_match(&Regex::new("^(F|f)(U|u).*$").unwrap(), &input) {
		pattern_type = "full";
	} else {
		pattern_type = "third";
	}
	println!("Using type: [{}]", pattern_type);

	// Get address pattern
	println!("");
	println!("What pattern to look for?");
	println!("    - Must not include 'I', 'O', 'l' ");
	println!("    - Must be ASCII or a Regex pattern");
	println!("    - [48hinto...] would match if \"^48hinto.*$\" was typed");
	println!("    - [44hinto...|48h1nto...] would match if \"^4(4|8)h(i|1)nto.*$\" was typed");
	println!("    - Rust regex patterns are used: https://docs.rs/regex/latest/regex ");
	print!("Pattern: ");
	io::stdout().flush().unwrap();
	let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed read line");
	// Strip newline off input
	if input.ends_with('\n') {
		input.pop();
		if input.ends_with('\r') {
			input.pop();
		}
	}
	let pattern;
	if pattern_type == "third" {
		pattern = format!("^..{}.*$", input);
		if Regex::is_match(&Regex::new("(I|O|l)").unwrap(), &pattern) {
			println!("Pattern cannot contain 'I', 'O', or 'l'");
			exit(1);
		}
	} else {
		pattern = format!("{}", input);
	}

	// Print input values
	println!("");
	print!("Using [{}] cores, looking for pattern [{}] with type [{}]... ", cores, Regex::new(&pattern).unwrap(), pattern_type);
	io::stdout().flush().unwrap();

	// Set runtime values
	let one = Scalar::one();        // Set scalar with value [1]
	let mut tries = 1;              // Interations
	let network = Network::Mainnet; // Monero network = Mainnet
	let now = Instant::now();       // Set timer

	// Start
	// [full] pattern 1-95 char matching
	if pattern_type == "full" {
		for _i in 1..=cores {
			let regex = Regex::new(&pattern).unwrap();
			let mut scalar = Scalar::random(&mut OsRng);
			thread::spawn(move|| loop {
				let private = PrivateKey { scalar: scalar, };
				let public = PublicKey::from_private_key(&private);
				if Regex::is_match(&regex, &Address::standard(network, public, public).to_string()) {
					println!("\n");
					println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
					println!("Found in [{}] tries in [{:?}]!\n", tries * cores, now.elapsed());
				    println!("Private Spend Key | {}", private);
				    println!("Standard Address  | {}\n", Address::standard(network, public, public).to_string());
					println!("Recover with: monero-wallet-cli --generate-from-spend-key");
					println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
					exit(0);
				}
				tries += 1;
				scalar.add_assign(one);
			});
		}
	// else, 1-43/3-43 char matching
	} else {
		for _i in 1..=cores {
			let regex = Regex::new(&pattern).unwrap();
			let mut scalar = Scalar::random(&mut OsRng);
			thread::spawn(move|| loop {
				let private = PrivateKey { scalar: scalar, };
				if Regex::is_match(&regex, &base58_monero::encode(&[[18].as_ref(), &PublicKey::from_private_key(&private).to_bytes()].concat()).unwrap()) {
					let public = PublicKey::from_private_key(&private);
					println!("\n");
					println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
					println!("Found in [{}] tries in [{:?}]!\n", tries * cores, now.elapsed());
				    println!("Private Spend Key | {}", private);
				    println!("Standard Address  | {}\n", Address::standard(network, public, public).to_string());
					println!("Recover with: monero-wallet-cli --generate-from-spend-key");
					println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
					exit(0);
				}
				tries += 1;
				scalar.add_assign(one);
			});
		}
	}
	thread::park();
}
