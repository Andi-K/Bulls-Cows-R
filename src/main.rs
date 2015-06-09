#[macro_use]
extern crate kiss_ui;
use kiss_ui::button::Button;

extern crate current;
use current::{ Current, CurrentGuard };

extern crate term_painter;

mod settings;
mod game;

// mod gui;

fn main() {

	let cfg = settings::Settings::load("settings");
	
	let mut game_data = game::Game::new(cfg.digits, cfg.min, cfg.max);

	if cfg.useGui {
		mod gui;
		let guard = CurrentGuard::new(&mut game_data);
		let mut ui = gui::Gui::new();
		ui.run();
	} else {
		mod tui;
		let mut ui = tui::Tui::new(&mut game_data);
		ui.run();
	};
	
/*	let mut ui = if cfg.useGui {
		mod gui;
		let guard = CurrentGuard::new(&mut game_data);
		gui::Gui::new()
	} else {
		mod tui;
		tui::Tui::new(&mut game_data)
	};
	
	ui.run();
*/
}
