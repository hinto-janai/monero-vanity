//---------------------------------------------------------------------------------------------------- Constants
/// `monero-vanity` version
///
/// It uses `CARGO_PKG_VERSION`, or `version` found in `Cargo.toml`.
pub const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

/// `monero-vanity` + version
///
/// Just a string concatenating "monero-vanity" and the current version, e.g: `monero-vanity v1.0.0`
pub const NAME_VER: &str = concat!("monero-vanity v", env!("CARGO_PKG_VERSION"));

/// Current `git` commit.
pub const COMMIT: &str = include_str!("commit");

pub const VERSION_COMMIT: &str = concat!(
	"v",
	env!("CARGO_PKG_VERSION"),
	" | Commit: ",
	include_str!("commit"),
);

/// 1 second.
pub const SECOND: std::time::Duration = std::time::Duration::from_secs(1);

//---------------------------------------------------------------------------------------------------- Text
pub const THIRD: &str =
r#"monero-vanity automatically prefixes your input
with `^4.` and suffixes it with `.*$` so that
your PATTERN starts from the 3rd character
until the 43rd character of the address.

Example input: `hinto`
Actual regex used: `^4.hinto.*$`"#;

pub const FIRST: &str =
r#"This disables the prefixing + suffixing when using `Third`

Warning: this puts you in full control of the regex,
you can input any value, even an impossible one."#;

pub const STATS: & str = "Stats on the current/previous run.";

pub const HISTORY: & str = "The found addresses, and private spend/view keys.";

//---------------------------------------------------------------------------------------------------- `egui`
/// `egui` Colors.
pub const BONE:      egui::Color32 = egui::Color32::from_rgb(190, 190, 190);
pub const RED:       egui::Color32 = egui::Color32::from_rgb(230, 50, 50);
pub const GREEN:     egui::Color32 = egui::Color32::from_rgb(100, 230, 100);
pub const DARK_GRAY: egui::Color32 = egui::Color32::from_rgb(18, 18, 18);

/// App resolution.
pub const APP_RESOLUTION: [f32; 2] = [1000.0, 800.0];

/// App icon.
pub const ICON: &[u8] = include_bytes!("icon.png");

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
