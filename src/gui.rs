//---------------------------------------------------------------------------------------------------- Use
use crate::state::State;
use egui::{
	CentralPanel,
	Label,RichText,Button,
	FontFamily,FontId,TextStyle,Slider,
	Style,TextEdit,SelectableLabel,
	Frame,ScrollArea,
};
use crate::constants::{
	ICON,NAME_VER,
	BONE,GREEN,RED,
	APP_RESOLUTION,
	DARK_GRAY,
	THIRD,FIRST,
	STATS,HISTORY,
};
use crate::threads::{
	THREADS_MAX,
	THREADS_HALF,
};
use crate::pattern::PatternType;
use regex::Regex;
use std::time::Instant;
use std::fmt::Write;
use readable::{
	Unsigned,
	Time,
};

//---------------------------------------------------------------------------------------------------- Gui
#[derive(Debug)]
pub struct Gui {
	/// Channel to `GUI`.
	to: std::sync::mpsc::Sender::<(String, String, String)>,

	/// Channel from `worker` threads.
	from: std::sync::mpsc::Receiver::<(String, String, String)>,

	/// General State.
	state: State,

	/// Third vs First
	pattern_type: PatternType,

	/// Current user-input pattern.
	pattern: String,

	/// Old user-input pattern.
	old_pattern: String,

	/// Current user-input threads.
	threads: usize,

	/// Is the address pattern valid?
	regex_ok: bool,

	/// Why did the regex fail?
	regex_fail: &'static str,
}

impl Default for Gui {
	fn default() -> Self {
		let (to, from) = std::sync::mpsc::channel::<(String, String, String)>();

		Self {
			to,
			from,

			state: State::default(),
			pattern_type: PatternType::default(),
			pattern: String::new(),
			old_pattern: String::new(),
			threads: *THREADS_HALF,
			regex_ok: false,
			regex_fail: "Address pattern must not be empty",
		}
	}
}

//---------------------------------------------------------------------------------------------------- `eframe` data init.
impl Gui {
	#[inline(always)]
	fn init_style() -> egui::Style {
		let style = Style {
			text_styles: [
				(TextStyle::Small,     FontId::new(15.0, FontFamily::Monospace)),
				(TextStyle::Body,      FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Button,    FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Monospace, FontId::new(20.0, FontFamily::Monospace)),
				(TextStyle::Heading,   FontId::new(40.0, FontFamily::Monospace)),
			].into(),
			..Default::default()
		};

		style
	}

	// Sets the initial options for native rendering with eframe
	pub fn options() -> eframe::NativeOptions {
		// Icon
		// SAFETY: This image is known at compile-time. It should never fail.
		let icon = image::load_from_memory(ICON).unwrap().to_rgba8();
		let (width, height) = icon.dimensions();
		let icon_data = Some(eframe::IconData {
			rgba: icon.into_raw(),
			width,
			height,
		});

		// The rest
		let options = eframe::NativeOptions {
			min_window_size: Some(egui::Vec2::from(APP_RESOLUTION)),
			initial_window_size: Some(egui::Vec2::from(APP_RESOLUTION)),
			follow_system_theme: false,
			default_theme: eframe::Theme::Dark,
			icon_data,
			..Default::default()
		};

		options
	}

	#[inline(always)]
	pub fn init(cc: &eframe::CreationContext<'_>) -> Self {
		let gui = Self::default();

		// Style
		cc.egui_ctx.set_style(Self::init_style());

		// Done.
		gui
	}
}

//---------------------------------------------------------------------------------------------------- `egui` event loop.
impl eframe::App for Gui {
    //-------------------------------------------------------------------------------- On exit.
    #[inline(always)]
    fn on_close_event(&mut self) -> bool {
		true
    }

	//-------------------------------------------------------------------------------- Main event loop.
	#[inline(always)]
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		// Max framerate.
		ctx.request_repaint();

		// Set global available width/height.
		let rect   = ctx.available_rect();
		let width  = rect.width() - 15.0;
		let height = rect.height();
		let text   = height / 25.0;

		// Set global stats.
		if self.state.iterating {
			self.state.elapsed = Time::from(self.state.start.elapsed());
			self.state.speed   = crate::speed::calculate(&self.state.start, self.state.iter.load(std::sync::atomic::Ordering::SeqCst));
		}
		let iter = Unsigned::from(self.state.iter.load(std::sync::atomic::Ordering::SeqCst));

		// Central Panel.
		CentralPanel::default().show(ctx, |ui| {
			//-------------------------------------------------- Title.
			let label = Label::new(
				RichText::new(NAME_VER)
				.heading()
				.color(BONE)
			);
			ui.add_sized([width, text], label);
			ui.add_space(10.0);
			ui.separator();
			ui.add_space(10.0);

			//-------------------------------------------------- User input address pattern.
			// Regex Check.
			if self.pattern != self.old_pattern {
				self.old_pattern = self.pattern.clone();
				match crate::regexes::validate(&self.pattern) {
					Some(fail) => {
						self.regex_ok   = false;
						self.regex_fail = fail;
					},
					None => self.regex_ok = true,
				};
			}

			ui.horizontal(|ui| {
				ui.set_enabled(!self.state.iterating);

				// TextEdit.
				let edit = TextEdit::singleline(&mut self.pattern)
					.hint_text("Enter address pattern, e.g: `hinto` would find an address like: `44hinto...`")
					.desired_width(width - 25.0);

				// Regex checkmark.
				if self.regex_ok {
					ui.add_sized([width - 25.0, text], edit);
					ui.add_sized([5.0, text], Label::new(RichText::new("✔").color(GREEN)));
				} else {
					ui.add_sized([width - 25.0, text], edit).on_hover_text(self.regex_fail);
					ui.add_sized([5.0, text], Label::new(RichText::new("❌").color(RED)));
				};
			});

			//-------------------------------------------------- PatternType.
			ui.add_space(10.0);
			ui.group(|ui| { ui.horizontal(|ui| {
				ui.set_enabled(!self.state.iterating);
				let width = (width / 2.0) - 10.0;
				if ui.add_sized([width, text], SelectableLabel::new(self.pattern_type == PatternType::Third, "Third (basic)")).on_hover_text(THIRD).clicked() {
					self.pattern_type = PatternType::Third;
				}
				if ui.add_sized([width, text], SelectableLabel::new(self.pattern_type == PatternType::First, "First (advanced)")).on_hover_text(FIRST).clicked() {
					self.pattern_type = PatternType::First;
				}
			})});

			//-------------------------------------------------- Threads.
			ui.add_space(10.0);
			ui.scope(|ui| {
				ui.set_enabled(!self.state.iterating);
				let width = width - 50.0;
				ui.spacing_mut().slider_width = width;
				ui.add_sized([width, text], Slider::new(&mut self.threads, 1..=*THREADS_MAX));
			});

			//-------------------------------------------------- Start/Stop.
			ui.add_space(10.0);
			ui.horizontal(|ui| {
				let w = (width / 2.0) - 5.0;
				ui.scope(|ui| {
					ui.set_enabled(!self.state.iterating && self.regex_ok);
					if ui.add_sized([w, text], Button::new("Start")).clicked() {
						// Start.
						let regex = match self.pattern_type {
							PatternType::Third => format!("^4.{}.*$", self.pattern),
							PatternType::First => self.pattern.to_string(),
						};

						self.state.die.store(false, std::sync::atomic::Ordering::SeqCst);
						self.state.iter.store(0, std::sync::atomic::Ordering::SeqCst);
						self.state.threads        = self.threads;
						self.state.pattern        = Regex::new(&regex).unwrap();
						self.state.pattern_string = regex;
						self.state.iterating      = true;
						self.state.start          = Instant::now();
						crate::address::spawn_workers(
							self.threads,
							&self.to,
							&self.state.iter,
							&self.state.die,
							&self.state.pattern,
						);
					}
				});
				ui.scope(|ui| {
					ui.set_enabled(self.state.iterating);
					if ui.add_sized([w, text], Button::new("Stop")).clicked() {
						// Stop.
						self.state.die.store(true, std::sync::atomic::Ordering::SeqCst);
						self.state.iterating = false;
					}
				});
			});

			//-------------------------------------------------- Stats.
			ui.add_space(text);
			let label = Label::new(
				RichText::new("Stats")
				.underline()
				.color(BONE)
			);
			ui.add_sized([width, text], label).on_hover_text(STATS);

			ui.add_space(5.0);

			egui::Frame::none().fill(DARK_GRAY).show(ui, |ui| {
				let results = format!(
					"Speed   | {} keys per second\nTries   | {}\nElapsed | {}\nThreads | {}\nPattern | {}",
					Unsigned::from(self.state.speed),
					iter,
					self.state.elapsed,
					Unsigned::from(self.state.threads),
					self.state.pattern,
				);
				ui.add_sized([width, text], TextEdit::multiline(&mut results.as_str()));
			});

			//-------------------------------------------------- History.
			ui.add_space(text);
			let label = Label::new(
				RichText::new("History")
				.underline()
				.color(BONE)
			);
			ui.add_sized([width, text], label).on_hover_text(HISTORY);

			ui.add_space(5.0);
			Frame::none().fill(DARK_GRAY).show(ui, |ui| {
				let width = width - 12.0;
				let height = ui.available_height();
				let scroll = ScrollArea::vertical()
					.stick_to_bottom(true)
					.always_show_scroll(true)
					.max_width(width)
					.max_height(height)
					.auto_shrink([false; 2]);

				scroll.show_viewport(ui, |ui, _| {
					ui.add_sized([width, height], TextEdit::multiline(&mut self.state.history.as_str()).text_style(TextStyle::Small));
				});
			});

			//-------------------------------------------------- Check for message.
			if let Ok(msg) = self.from.try_recv() {
				let iter = self.state.iter.load(std::sync::atomic::Ordering::SeqCst);

				writeln!(
					self.state.history,
					"Address   | {}\nSpend Key | {}\nView Key  | {}\nSpeed     | {} keys per second\nTries     | {}\n",
					msg.0,
					msg.1,
					msg.2,
					Unsigned::from(crate::speed::calculate(&self.state.start, iter)),
					Unsigned::from(iter),
				);

				self.state.die.store(true, std::sync::atomic::Ordering::SeqCst);
				self.state.iterating = false;
			}
		});
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
