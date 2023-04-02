//---------------------------------------------------------------------------------------------------- PatternType
use clap::ValueEnum;

//---------------------------------------------------------------------------------------------------- PatternType
#[derive(ValueEnum,Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum PatternType {
	#[default]
	Third,
	First,
}

impl std::fmt::Display for PatternType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
