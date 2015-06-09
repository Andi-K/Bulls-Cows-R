extern crate toml;
use self::toml::{Encoder, Value};

use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};


pub struct Settings {
	pub min: u8,
	pub max: u8,
	pub digits: u8,
	pub useGui: bool,
}

impl<'a> Settings {
	pub fn load(name: &str) -> Settings {
		let path = Path::new(name).with_extension("toml");
		let fileName = path.file_name().unwrap();
		let mut toml = String::new();
	    let _ = match File::open(fileName) {
	        Ok(ref mut rm) => { let _ = rm.read_to_string(&mut toml); },
	        Err(e) => { println!("Can't load configuration file '{}', use defaults.\n({:?})", path.display(), e); },
	    };
		
		let lookup_int = |var: &toml::Value, path: &str, default: i64| -> i64 {
			(*var).lookup(path).and_then(|x| x.as_integer()).unwrap_or(default)
		};
	
		let cnf: toml::Value = match toml.parse() {
	        Ok(v) => { v },
	        Err(e) => {
	        	println!("Can't parse configuration file '{}', use defaults.\n({:?})", path.display(), e); 
	        	toml::Value::String("".to_string())
        	},
	    };
		
		Settings {
			min: lookup_int(&cnf, "game.min", 1) as u8,
			max: lookup_int(&cnf, "game.max", 9) as u8,
			digits: lookup_int(&cnf, "game.digits", 4) as u8,
			useGui: cnf.lookup("UI.use_gui").and_then(|x| x.as_bool()).unwrap_or(false),
		}
	}
}
