//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use regex::Regex;

//---------------------------------------------------------------------------------------------------- Regex
/// Checks a `Regex` for Monero address validity.
///
/// If `None` is returned, it is _most likely_ ok.
/// Users can still craft impossible Regexes but
/// this makes the obvious impossible ones go away.
pub fn validate(s: &str) -> Option<&'static str> {
	if s.is_empty() {
		return Some("Address pattern must not be empty");
	} else if s.contains('I') {
		return Some("Address pattern must not contain 'I'");
	} else if s.contains('O') {
		return Some("Address pattern must not contain 'O'");
	} else if s.contains('l') {
		return Some("Address pattern must not contain 'l'");
	} else if s.contains('0') {
		return Some("Address pattern must not contain '0'");
	} else if s.contains('+') {
		return Some("Address pattern must not contain '+'");
	} else if s.contains('/') {
		return Some("Address pattern must not contain '/'");
	}
	match Regex::new(&s) {
		Ok(_)  => None,
		Err(_) => Some("Regex failed to build"),
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
