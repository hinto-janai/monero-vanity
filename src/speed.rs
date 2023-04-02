//---------------------------------------------------------------------------------------------------- __NAME__
#[inline(always)]
/// Calculate speed.
pub fn calculate(instant: &std::time::Instant, tries: u64) -> u64 {
	let elapsed = instant.elapsed().as_secs_f64();
	let speed   = (tries as f64) / elapsed;

	speed as u64
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
