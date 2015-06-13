
pub struct Game {
	min: u8,
	max: u8,
	items: Vec<String>,
	digits: u8,
	bulls_set: bool,
	bulls: u8,
	// cows: u8,
	turn: u8,
	max_turns: u8,
	all_combs: Vec<Vec<String>>,
	pub possible_guesses : Vec<Vec<String>>,
	guess: Vec<String>,
}

impl<'a> Game {
	pub fn new(digits: u8, items: Vec<String>, min: u8, max: u8)-> Game  {
		let mut all_combs = Game::gnr_all_combs(digits, items.clone(), min, max);
		Game {
			min: min,
			max: max,
			items: items,
			digits: digits,
			bulls_set: false,
			bulls: 0,
//			cows: 0,
			turn: 0,
			max_turns: 8,
			all_combs: all_combs.clone(),
			guess: all_combs.pop().expect(GPM!()),
			possible_guesses : all_combs,
		}
		
	}
	
	pub fn reset(&mut self)  {
		let mut all_combs = self.all_combs.clone();
		self.bulls_set = false;
		self.bulls = 0;
		self.turn = 0;
		self.guess = all_combs.pop().expect(GPM!());
		self.possible_guesses = all_combs;
	}

	pub fn is_bulls_set(&mut self) -> bool { self.bulls_set }
	pub fn get_digits(&mut self) -> u8 { self.digits }
	pub fn get_bulls(&mut self) -> u8 { self.bulls }
	pub fn get_turn(&mut self) -> u8 { self.turn }
	pub fn get_max_turns(&mut self) -> u8 { self.max_turns }
	pub fn count(&mut self) -> usize { self.possible_guesses.iter().count() }
	pub fn get_guess(&mut self) -> Vec<String> { self.guess.clone() }

	pub fn set_bulls(&mut self, x: u8) -> bool {
		if self.bulls_set || x > self.digits { return false; }
		self.bulls = x;
		self.bulls_set = true;
		true
	}

	/// calculate the `given` Vec<String> vs the current guess
	///
	/// returns the Score as (Bulls, Cows)
	fn calculate_score(given: &Vec<String>, guess: &Vec<String>) -> (u8, u8) {
		let mut bulls = 0;
		let mut cows = 0;
//		let mut guess = self.get_guess(); //.iter();
		let mut index = 0;
		for given_str in given.iter() {
			let pos: Option<u8> = {
				let mut result = None;
				let mut p = 0;
				for guess_str in guess.clone() {
					if &given_str[..] == &guess_str[..] { result = Some(p); break }
					p += 1;
				}
				result
			};

			match pos {
				None => (),
				Some(p) if p == index
					=> bulls += 1,
				Some(_) =>
					cows += 1,
			}
			
			index += 1;
		}
		return (bulls, cows);
	}

	pub fn set_cows(&mut self, cows: u8) -> bool {
		if !self.bulls_set || cows + self.bulls > self.digits { return false; }
		
		let guess = self.guess.clone();
		let bulls = self.bulls;
//		let guess = self.guess;

		// eliminate patterns with non-matching scores
		self.possible_guesses.retain(|ref x| (bulls, cows) == Game::calculate_score(&guess, &x));
		self.guess = self.possible_guesses.pop()
			.unwrap_or(vec!["-".to_string(); self.digits as usize]);

		self.bulls = 0;
		self.bulls_set = false;
		self.turn += 1;
		true
	}

	/// BUG: digits is ignored and fixed to 4
	fn gnr_all_combs(digits: u8, items: Vec<String>, min: u8, max: u8) -> Vec<Vec<String>> {
		// TODO: 
		// if stable
		// use  std::slice::Permutations;
		// this is just a temporay solution 
		let mut guesses: Vec<Vec<String>> = vec![];
		for p1 in &items {
			for p2 in &items {
				if *p2 == *p1 {continue};
				for p3 in &items {
					if *p3 == *p2 || *p3 == *p1 {continue};
					for p4 in &items {
						if *p4 == *p3 || *p4 == *p2 || *p4 == *p1 {continue};
						let mut item: Vec<String> = vec!();
						item.push((*p1).to_string().clone());
						item.push((*p2).to_string().clone());
						item.push((*p3).to_string().clone());
						item.push((*p4).to_string().clone());
						guesses.push(item);
					}
				}
			}
		}
		guesses
	}
	
	pub fn arr_to_str(a:&Vec<String>) -> String {
		let mut s = String::new();
		for i in a.iter() {
			s.push_str(&i.to_string());
			s.push_str(" ");
		}
		s.pop();
		 return s;
	}
	
	pub fn vec_to_str(v: &Vec<Vec<String>>) -> String {
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