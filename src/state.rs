//---------------------------------------------------------------------------------------------------- Use
use regex::Regex;
use std::sync::{
	Arc,
};
use std::sync::atomic::{
	AtomicU64,
	AtomicBool,
};
use std::time::Instant;

//---------------------------------------------------------------------------------------------------- State
#[derive(Debug)]
pub struct State {
	// Set-up variables.
	/// How many threads to use.
	pub threads: usize,
	/// The address regex pattern to look for.
	pub pattern: Regex,
	/// The address regex pattern to look for (as a String).
	pub pattern_string: String,
	/// How many iterations are we on?
	pub iter: Arc<AtomicU64>,

	// Runtime variables.
	/// Are we currently iterating?
	pub iterating: bool,
	/// What is our (iteration/per second) speed?
	pub speed: u64,
	/// When did we start?
	pub start: Instant,
	/// How many seconds since starting?
	pub elapsed: readable::Time,
	/// Found Private Spend Key(s).
	pub history: String,

	// Thread signals.
	/// Should all threads stop and die?
	pub die: Arc<AtomicBool>,
}

impl Default for State {
	fn default() -> Self {
		Self {
			threads: 1,
			pattern: Regex::new("").unwrap(),
			pattern_string: "".to_string(),
			iter: Arc::new(AtomicU64::new(0)),
			iterating: false,
			speed: 0,
			start: Instant::now(),
			elapsed: readable::Time::from(0_u8),
			history: "".to_string(),
			die: Arc::new(AtomicBool::new(false)),
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
