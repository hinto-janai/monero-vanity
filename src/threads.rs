//---------------------------------------------------------------------------------------------------- Threads
lazy_static::lazy_static! {
	pub static ref THREADS_MAX: usize = {
		match std::thread::available_parallelism() {
			Ok(u)  => u.get(),
			Err(e) => { eprintln!("Could not detect available threads: {}. Defaulting to 1", e); 1 },
		}
	};

	pub static ref THREADS_80: usize = {
		match *THREADS_MAX {
			// Special cases (low thread-count).
			1 => 1,
			2 => 1,
			3 => 2,
			4 => 3,

			// Around 80%.
			_ => (*THREADS_MAX as f64 * 0.8).floor() as usize,
		}
	};

	pub static ref THREADS_HALF: usize = {
		(*THREADS_MAX as f64 / 2.0).floor() as usize
	};
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
