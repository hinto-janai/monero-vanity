mod address;
mod cli;
mod constants;
mod gui;
mod state;
mod threads;
mod regexes;
mod pattern;
mod speed;

fn main() {
	// Handle `CLI`.
	if std::env::args().collect::<Vec<String>>().len() > 1 {
		cli::Cli::handle_args();
	}

	// Handle `GUI`.
	eframe::run_native(
		crate::constants::NAME_VER,
		gui::Gui::options(),
		Box::new(|cc| Box::new(gui::Gui::init(cc)))
	).expect("eframe::run_native() failed");
}
