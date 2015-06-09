
pub struct Game {
 min: u8,
 max: u8,
 digits: u8,
 bulls_set: bool,
 bulls: u8,
// cows: u8,
 round: u8,
all_combs: Vec<[u8; 4]>,
pub possible_guesses : Vec<[u8; 4]>,
try: [u8; 4],
}

impl<'a> Game {
    pub fn new(digits: u8, min: u8, max: u8)-> Game  {
    	let mut all_combs = Game::gnr_all_combs(digits, min, max);
    	Game {
			min: min,
			max: max,
			digits: digits,
			bulls_set: false,
			bulls: 0,
//			cows: 0,
			round: 0,
    		all_combs: all_combs.clone(),
    		try: all_combs.pop().unwrap(),
    		possible_guesses : all_combs,
    	}
    	
    }
    
    pub fn reset(&mut self)  {
    	let mut all_combs = self.all_combs.clone();
		self.bulls_set = false;
		self.bulls = 0;
		self.round = 0;
		self.try = all_combs.pop().unwrap();
		self.possible_guesses = all_combs;
    }

   	pub fn is_bulls_set(&mut self) -> bool { self.bulls_set }
	pub fn get_digits(&mut self) -> u8 { self.digits }
	pub fn get_bulls(&mut self) -> u8 { self.bulls }
	pub fn get_round(&mut self) -> u8 { self.round }
	pub fn get_try(&mut self) -> [u8; 4] { self.try }
	pub fn count(&mut self) -> usize { self.possible_guesses.iter().count() }

	pub fn set_bulls(&mut self, x: u8) -> bool {
		if self.bulls_set || x > self.digits { return false; }
		self.bulls = x;
		self.bulls_set = true;
		true
	}
fn calculate_score(given_digits: &[u8], guessed_digits: &[u8]) -> (u8, u8) {
    let mut bulls = 0;
    let mut cows = 0;
    for i in (0..4) {
        let pos = guessed_digits.iter().position(|&a| a == given_digits[i]);
        
        match pos {
            None => (),
            Some(p) if p == i => bulls += 1,
            Some(_) => cows += 1,
        }
    }
    return (bulls, cows);
}

	pub fn set_cows(&mut self, cows: u8) -> bool {
		if !self.bulls_set || cows + self.bulls > self.digits { return false; }
		
		let bulls = self.bulls;
		let try = self.try;
		// eliminate patterns with non-matching scores
		self.possible_guesses.retain(|&x| (bulls, cows) == Game::calculate_score(&try, &x));
		self.try = self.possible_guesses.pop().unwrap_or([0,0,0,0]);

/*			
			match self.possible_guesses.iter().count() {
				0 => { false },
				2 ... 5 => {
					self.try = self.possible_guesses.pop().unwrap();
					print!("Es könnten noch {}\nund {}. sein! Versuch letztere Combi: ",
                    Color::BrightBlack.paint(vec_to_str(&possible_guesses)), (Color::Yellow.paint(arr_to_str(&guesses))));
					stdout().flush().ok().expect("Could not flush stdout");
                },
				1 => {
					guesses = possible_guesses.pop().unwrap();
                println!("\nEs ist {}! :)", (Color::Green.paint(arr_to_str(&guesses)))); break },
				_ => {
					guesses = possible_guesses.pop().unwrap();
					print!("Und bei {}? ({} mögliche Kombinationen): ",
                    (Color::Yellow.paint(arr_to_str(&guesses))), possible_guesses.iter().count() + 1);
					stdout().flush().ok().expect("Could not flush stdout");
                },
            }
*/
		self.bulls = 0;
		self.bulls_set = false;
		self.round += 1;
		true
	}

	/// BUG: digits is ignored and fixed to 4
	fn gnr_all_combs(digits: u8, min: u8, max: u8) -> Vec<[u8; 4]> { // &mut self
		// TODO: 
		// if stable
		// use  std::slice::Permutations;
		// this is just a temporay soluchen 
		let mut guesses: Vec<[u8; 4]> = vec![];
		let mut numbers: Vec<u8> = (min .. max).collect();
		numbers.sort_by(|a, b| b.cmp(a));
	//	let numbers: Vec<u8> = vec!(5, 6, 7, 8, 9, 4, 3, 2, 1);
		for p1 in &numbers {
			for p2 in &numbers {
				if *p2 == *p1 {continue};
				for p3 in &numbers {
					if *p3 == *p2 || *p3 == *p1 {continue};
					for p4 in &numbers {
		                if *p4 == *p3 || *p4 == *p2 || *p4 == *p1 {continue};
		                guesses.push([*p1, *p2, *p3, *p4]);
		            }
		        }
		    }
		}
		guesses
	}
	
	pub fn arr_to_str(a:&[u8]) -> String {
	    let mut s = String::new();
	    for i in a.iter() {
	        s.push_str(&i.to_string());
	        s.push_str(" ");
	    }
	    s.pop();
	     return s;
	}
	
	pub fn vec_to_str(v: &Vec<[u8; 4]>) -> String {
	    let mut s = String::new();
	    for i in v.iter() {
	        s.push_str(&Game::arr_to_str(i)[..]);
	        s.push_str(", ");
	    }
	    s.pop();
	    s.pop();
	    return s;
	}


}