// extern crate term_painter;
use term_painter::ToStyle;
use term_painter::Color;
use term_painter::Attr;

use std::io::stdin;
use std::io::Write;
use std::io::stdout;
use std::fmt;

use game;
use game::Game;

use std;
use std::collections::HashMap;
use std::boxed::Box;
use std::str::FromStr;

/*
extern crate current;
use current::{ Current, CurrentGuard };
macro_rules! game (
	() => (unsafe { &mut *Current::<game::Game>::new() })
);
*/

pub struct Tui<'a> {
	game_data: &'a mut game::Game,
}

impl<'a> Tui<'a> {
	
	pub fn new(data: &'a mut game::Game)-> Tui  {
    	Tui {
			game_data: data,
    	}
    }

	pub fn run (&mut self) {
		println!("run");
		let mut guesses = self.game_data.get_try();
		loop {
	        
	        print!("Wie viele {} und {} Treffer gibt es bei {}?\n(Einfach nur die Zahlen mit einem Trennzeichen eingeben) : ",
	        (Color::Green.paint("grüne")), (Color::Red.paint("rote")),
	        (Color::Yellow.paint(Game::arr_to_str(&guesses))));
			stdout().flush().ok().expect("Could not flush stdout");
			//		print!("{}", (Color::Green.paint("")));
			
			let mut answer_string = String::new();
			let mut reader = std::io::stdin();
			loop {
				answer_string = "".to_string();
				let _ = reader.read_line(&mut answer_string).unwrap();
				// println!("Eingabe ist {:?}! :)", answer_string);
				if answer_string.trim().to_string() == "x".to_string() { break }
				let mut answer_chars = answer_string.trim().chars();
				
				if answer_string.trim().chars().count() != 3 {
					print!("Bitte {} für Abbrechen oder Grüne*irgendwas*Rote (\"{}\" oder \"{}\") eingeben: ",
	                (Color::BrightBlack.paint("x")), (Color::BrightBlack.paint("0,2")), (Color::BrightBlack.paint("1 2")));
					stdout().flush().ok().expect("Could not flush stdout");
					continue;
	            }
				let (bulls, cows) = parse_answer_string(&mut answer_chars);
				if bulls + cows > 4 {
					print!("{}, bitte nochmal: ", (Color::Red.paint("Fehlerhafte Eingabe")));
					stdout().flush().ok().expect("Could not flush stdout");
					continue;
	            }
				
				if bulls == 4 {
					println!("{}", (Color::Green.paint("Super :D"))); 
					break;
	            }
				
			self.game_data.set_bulls(bulls);
			
			let last_try = self.game_data.get_try();
	    	self.game_data.set_cows(cows);
				
				match self.game_data.count() {
					0 => {
	                println!("{}", (Color::Red.paint("Kann nicht sein! Abbruch..."))); break},
					2 ... 5 => {
						guesses = self.game_data.possible_guesses.pop().unwrap();
						print!("Es könnten noch {}\nund {}. sein! Versuch letztere Combi: ",
	                    Color::BrightBlack.paint(Game::vec_to_str(&self.game_data.possible_guesses)), (Color::Yellow.paint(Game::arr_to_str(&guesses))));
						stdout().flush().ok().expect("Could not flush stdout");
	                },
					1 => {
						guesses = self.game_data.possible_guesses.pop().unwrap();
	                println!("\nEs ist {}! :)", (Color::Green.paint(Game::arr_to_str(&guesses)))); break },
					_ => {
						guesses = self.game_data.possible_guesses.pop().unwrap();
						print!("Und bei {}? ({} mögliche Kombinationen): ",
	                    (Color::Yellow.paint(Game::arr_to_str(&guesses))), self.game_data.possible_guesses.iter().count() + 1);
						stdout().flush().ok().expect("Could not flush stdout");
	                },
	            }
				
	        }
			
			println!("\nNochmal? ({} und {} beenden das Programm)", (Color::BrightBlack.paint("x")), (Color::BrightBlack.paint("n")));
			let mut answer_string = String::new();
			let mut reader = std::io::stdin();
			let _ = reader.read_line(&mut answer_string).unwrap();
			// println!("Eingabe ist {:?}! :)", answer_string);
			match answer_string.trim().chars().nth(0).unwrap() {
				'x' | 'n' => { break},
				_ => {}
	        }
		}
	}
}

fn parse_answer_string(chars: &mut std::str::Chars) -> (u8, u8) {
	let mut bulls: u8 = 99;
	let mut cows: u8 = 99;
	
	match chars.nth(0).unwrap() {
		'0' => {bulls = 0} ,
		'1' => {bulls = 1} ,
		'2' => {bulls = 2} ,
		'3' => {bulls = 3} ,
		'4' => {bulls = 4} ,
		_ => {},
    }
	
	match chars.nth(1).unwrap() {
		'0' => {cows = 0} ,
		'1' => {cows = 1} ,
		'2' => {cows = 2} ,
		'3' => {cows = 3} ,
		'4' => {cows = 4} ,
		_ => {},
    }
	// println!("B: {} c:{}", bulls, cows);
	
	return (bulls, cows);
}



