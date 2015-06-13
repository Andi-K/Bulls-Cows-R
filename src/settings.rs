extern crate toml;
use self::toml::{Value, Decoder};

use std::io::prelude::*;
use std::fs::File;
use std::path::{Path};

extern crate rustc_serialize;
use self::rustc_serialize::Decodable;

pub struct Settings {
	pub min: u8,
	pub max: u8,
	pub digits: u8,
	pub items: Vec<String>,
	pub uniquire: bool,
	pub max_turns: u8,
	pub use_gui: bool,
}

impl<'a> Settings {
	pub fn load(name: &str) -> Settings {

		let lookup_int = |var: &toml::Value, path: &str, default: i64| -> i64 {
			(*var).lookup(path).and_then(|x| x.as_integer()).unwrap_or(default)
		};
		let lookup_bool = |var: &toml::Value, path: &str, default: bool| -> bool {
			(*var).lookup(path).and_then(|x| x.as_bool()).unwrap_or(default)
		};
	
		let path = Path::new(name).with_extension("toml");
		let file_name = path.file_name().expect(GPM!());
		let mut toml = String::new();
		let _ = match File::open(file_name) {
			Ok(ref mut rm) => { let _ = rm.read_to_string(&mut toml); },
			Err(e) => { println!("Can't load configuration file '{}', use defaults.\n({:?})", path.display(), e); },
		};
		
		let cnf: toml::Value = match toml.parse() {
			Ok(v) => { v },
			Err(e) => {
				println!("Can't parse configuration file '{}', use defaults.\n({:?})", path.display(), e); 
				toml::Value::String("".to_string())
			},
		};
		
		let mut decoder = toml::Decoder::new(
			cnf.lookup("game.foo").unwrap_or(&toml::Value::String("".to_string())).clone());

		let default_items = (1 .. 9).map( |d| { d.to_string() }).collect();

		Settings {
			items: Decodable::decode(&mut decoder)
				.unwrap_or( default_items), // println!("Items"); 
			min: lookup_int(&cnf, "game.min", 1) as u8,
			max: lookup_int(&cnf, "game.max", 9) as u8,
			max_turns: lookup_int(&cnf, "game.max_turns", 8) as u8,
			digits: 4, // TODO: get other values to work
//			digits: lookup_int(&cnf, "game.digits", 4) as u8,
			use_gui: lookup_bool(&cnf, "UI.use_gui", false),
			uniquire: lookup_bool(&cnf, "game.uniquire", true),
		}
	}
}
