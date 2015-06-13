#[macro_use]
extern crate kiss_ui;

extern crate current;
use current::CurrentGuard;

extern crate term_painter;

/// Generate a more usefull genetic panic massage
///
/// The massage *called 'Option::unwrap()' on a 'None' value [...] option.rs:362*
/// is hard to debug.
/// Replacing `.unwarp()` by `.expect(GPM!())` is a solution if you can't avoid the panic
/// by useing `.unwarp_or()` or something else.
#[macro_export]
macro_rules! GPM (
	() => (
		&format!("{}@{}:{} expected a value", file!(), line!(), column!())
	)
);


mod settings;
mod game;

fn main() {

	let cfg = settings::Settings::load("settings");
	
	let mut game_data = game::Game::new(cfg.digits, cfg.items, cfg.min, cfg.max);

	if cfg.use_gui {
		mod gui;
		// guard is used, don't remove!
		// we can use the game! macro as long as it life
		let guard = CurrentGuard::new(&mut game_data);
		let mut ui = gui::Gui::new();
		ui.run();
	} else {
		mod tui;
		let mut ui = tui::Tui::new(&mut game_data);
		ui.run();
	};
}
