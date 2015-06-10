#[macro_use]
extern crate kiss_ui;

extern crate current;
use current::CurrentGuard;

extern crate term_painter;

mod settings;
mod game;

// mod gui;

fn main() {

	let cfg = settings::Settings::load("settings");
	
	let mut game_data = game::Game::new(cfg.digits, cfg.min, cfg.max);

	if cfg.useGui {
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
