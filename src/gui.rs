use game;
use std::collections::HashMap;
use kiss_ui::button::Button;
use kiss_ui::base::BaseWidget;
use kiss_ui::container::{Horizontal, Vertical, Grid};
use kiss_ui::callback::{OnClick, Callback, CallbackStatus};
use kiss_ui::dialog::Dialog;
use kiss_ui::text::Label;
use kiss_ui::widget::Widget;
use kiss_ui::image::{Image, ImageContainer};
use kiss_ui::show_gui;
use kiss_ui::container::Orientation::Vertical as OVert;
use kiss_ui::dialog::{self, AlertPopupBuilder};

use std::boxed::Box;
use std::str::FromStr;

extern crate current;
use current::{ Current, CurrentGuard };
macro_rules! game (
	() => (unsafe { &mut *Current::<game::Game>::new() })
);


macro_rules! widget_by_name (
    ($dialog:ident, $w_type:ident, $name:ident) => (
	$dialog.get_child(&$name)
		.expect(&(format!("{} not found: {}", stringify!($w_type), $name))[..])
		.try_downcast::<$w_type>().ok()
		.expect(&(format!("Widget '{}' is not a {}", $name, stringify!($w_type))[..]))
    )
);



fn btn_by_name(window: Dialog, name: &str) -> Button {
//  let name = format!("{}{}", base_name, id);
//    let name = base_name.to_string() + &(id.to_string());
	window.get_child(&name)
		.expect(&(format!("Button not found: {}", name))[..])
		.try_downcast::<Button>().ok()
		.expect(&(format!("Widget '{}' is not a Button?", name)[..]))
}

pub struct Gui {
//	game_data: &'a mut game::Game,
	bulls_set: bool,
}

	    const ICON_WIDTH: u32 = 20;
	    const ICON_HEIGHT: u32 = 20;
	    const ICON_SIZE: usize = (ICON_HEIGHT * ICON_WIDTH) as usize;

impl<'a> Gui {
	
//	    let icon_gray: Vec<_> = vec![(155, 155, 155); ICON_SIZE];
//    	let icon_red: Vec<_> = vec![(255, 0, 0); ICON_SIZE];
//	    let icon_green: Vec<_> = vec![(0, 255, 0); ICON_SIZE];
	
        
	
//	pub fn new(data: &'a mut game::Game)-> Gui  {
	pub fn new()-> Gui  {
    	Gui {
			bulls_set: false,
//			game_data: data,
    	}
    }

	// Generate `x` Buttons with an name starting with `base_name`
	// , onclick: FnMut
	fn add_multi_btn (&mut self, labels : Vec<&str>, base_name : &str, vec : &mut Vec<BaseWidget>) -> HashMap<String, Box<Button>> {
		let mut names: HashMap<String, Box<Button>> = HashMap::with_capacity(labels.len());
		let mut i = 0;
	    	for label in labels {
	    		let name  = format!("Btn{}_{}", base_name, i);
	    		let btn = Box::new(
	    			Button::new()
	    			.set_name(&name)
	    			.set_label(label)
	//    			.set_onclick(onclick)
	    			);
				vec.push(btn.to_base());
				names.insert(name, btn);
				i += 1;
	   		}
		names
	}

	fn add_multi_lbl (&mut self, labels : Vec<&str>, base_name : &str, vec : &mut Vec<BaseWidget>) -> HashMap<String, Box<Label>> {
		let mut names: HashMap<String, Box<Label>> = HashMap::with_capacity(labels.len());
		let mut i = 0;
	    	for label in labels {
	    		let name  = format!("Lbl{}_{}", base_name, i);
	    		let btn = Box::new(
	    			Label::new(label)
	    			.set_name(&name)
	    			);
				vec.push(btn.to_base());
				names.insert(name, btn);
				i += 1;
	   		}
		names
	}

	pub fn run (&mut self) {
		show_gui(move || {
			let mut children_try = children![Label::new("Next try: ")];
			// Add *4* Buttons with names starting with *BtnTest* to
			let btns_try = self.add_multi_btn(vec!("1", "2", "3", "4"), "Next", &mut children_try);
			let mut children_input = children![Label::new("Bulls: ").set_name("LblInput")];
			let btns_input = self.add_multi_btn(vec!("0", "1", "2","3", "4"), "Input", &mut children_input);
			let mut children_output = children![];
			let mut outputHash: HashMap<u8, HashMap<String, Box<Label>>> = HashMap::with_capacity(7);
	        for i in (1..8u8) {
	        	outputHash.insert(i, self.add_multi_lbl(vec!("t: ", "[1, 2, 3, 4]", "", "", "", ""), &format!("Output_{}", i)[..], &mut children_output));
	        }
	/*
			let mut process_input = move |b: Button| {
				println!("bar");
	//       	let outputLine = outputHash.get(&1).expect("expected output line not found");
	
				game_data.set_min(3);
			};
	*/        
	
	        // 
	        for (_, btn) in btns_try {
	        	btn.set_enabled(false);
	        }
	
	        for (_, btn) in btns_input {
	        	btn.set_onclick(Gui::process_input);
	        }
	    let icon_gray: Vec<_> = vec![(155, 155, 155); ICON_SIZE];
	
	        // 
	        for i in (1..8u8) {
	        	let outputLine = outputHash.get(&i).expect("expected output line not found");
	        	outputLine.get(&format!("LblOutput_{}_0", i)[..]).expect("expected Label not found")
	        	.set_text("Try x: ").hide();
	        	outputLine.get(&format!("LblOutput_{}_1", i)[..]).expect("expected Label not found")
	        	.set_text("1234").hide();
		        for x in (2..6u8) {
		        	outputLine.get(&format!("LblOutput_{}_{}", i, x)[..]).expect("expect Label not found")
					.set_image(Image::new_rgb(ICON_WIDTH, ICON_HEIGHT, &icon_gray))
					.hide();
		        }
			}
	
	        let window = Dialog::new(
	        	Vertical::new(
	        		children![
			            Horizontal::new(children_try)
			            .set_elem_spacing_pixels(25),
			            Horizontal::new(children_input)
			            .set_elem_spacing_pixels(25),
			            Label::new(""),
			            Grid::new(children_output)
			            .set_ndiv(5)
			            .set_orientation(OVert),
		            ]
	            )
	        )
	        .set_title("revers Bulls 'n' Cows")
	//        .set_size_pixels(320, 500)
	        ;
	        window
		})
    }

	fn process_input(btn: Button) -> CallbackStatus {
		// no u8::from_char() ? -> dirty way
		let value: u8 = btn.get_name().expect("Button don't have a name?")
			.bytes().last().expect("Button with zero-leng-name?")
			- 48;
		let digits = game!().get_digits();
   		let dialog = btn.get_dialog().unwrap();
	
//		println!("Button # {:?} clicked! Mode: ",value);
//		let data = unsafe { &*Current::<game::Game>::new() };

		if game!().is_bulls_set() {
			// set cows
			let bulls = game!().get_bulls();
			let last_try = game!().get_try();
	    	if game!().set_cows(value) {
//			    let icon_green: Vec<_> = vec![(0, 255, 0); ICON_SIZE];
//			    let icon = Image::new_rgb(ICON_WIDTH, ICON_HEIGHT, &icon_green);

			match game!().count() {
				0 => {
					return Gui::show_dialog(dialog, "That can't be right",
							"Please check your answers!\n\nDo you like to play again?")
					},
				
/*				2 ... 5 => {
					self.try = self.possible_guesses.pop().unwrap();
					print!("Es könnten noch {}\nund {}. sein! Versuch letztere Combi: ",
                    Color::BrightBlack.paint(vec_to_str(&possible_guesses)), (Color::Yellow.paint(arr_to_str(&guesses))));
					stdout().flush().ok().expect("Could not flush stdout");
                },
*/				1 => {
					return Gui::show_dialog(dialog, "I know it!",
							&format!("It is {:?}\nDo you like to play again?",
								game!().get_try() )[..])
					}
				_ => {
                },
            }

		    	Gui::set_next(dialog);

		    	for i in (0 .. digits +1) {
		    		let name  = format!("BtnInput_{}", i);
			 		widget_by_name!(dialog, Button, name)
						.set_enabled(true)
					 // FIXME: set_image() don't work here
//				    	.set_image(icon)
//					    .set_label("")
					    ;
	    		}
		    	let name = "LblInput";
		 		widget_by_name!(dialog, Label, name).set_text("Bulls: ");
		 		
				let round = game!().get_round();
				
	    let icon_gray: Vec<_> = vec![(155, 155, 155); ICON_SIZE];
    	let icon_red: Vec<_> = vec![(255, 0, 0); ICON_SIZE];
	    let icon_green: Vec<_> = vec![(0, 255, 0); ICON_SIZE];
		    	let name = format!("LblOutput_{}_0", round);
		 		widget_by_name!(dialog, Label, name).set_text(&format!("Try {}: ", round)[..]).show();		
		    	let name = format!("LblOutput_{}_1", round);
		 		widget_by_name!(dialog, Label, name).set_text(&format!("{:?}: ", last_try)[..]).show();		
		        for i in (2 .. digits + 2) {
		        	let icon = if i < (bulls +2) { &icon_green }
		        		else if i < ( bulls + value +2) { &icon_red }
		        		else { &icon_gray };
/*		        	let icon = if i > ( digits - bulls +1) { &icon_green }
		        		else if i > ( digits - bulls - value +1) { &icon_red }
		        		else { &icon_gray };
*/			    	let name = format!("LblOutput_{}_{}", round, i);
		        	widget_by_name!(dialog, Label, name)
					.set_image(Image::new_rgb(ICON_WIDTH, ICON_HEIGHT, icon))
					.show();
		        }

	 		} else {
	 			dialog::message_popup("Warning", "Something went wrong!");
	 		}
		} else {
			// set bulls
			if digits == value {
				return Gui::show_dialog(dialog, "I won! :P", "Do you like to play again?")
			}
	    	if game!().set_bulls(value) {
//			    let icon_green: Vec<_> = vec![(0, 255, 0); ICON_SIZE];
//			    let icon = Image::new_rgb(ICON_WIDTH, ICON_HEIGHT, &icon_green);
		    	for i in ((digits - value +1) .. digits +1) {
		    		let name  = format!("BtnInput_{}", i);
			 		widget_by_name!(dialog, Button, name)
						.set_enabled(false)
					 // FIXME: set_image() don't work here
//				    	.set_image(icon)
//					    .set_label("")
					    ;
	    		}
		    	let name = "LblInput";
		 		widget_by_name!(dialog, Label, name).set_text("Cows: ");
	 		} else {
	 			dialog::message_popup("Warning", "Something went wrong!");
	 		}
    	}
		CallbackStatus::Default
	// , self.game_data.min
	   //    let dialog = caller.get_dialog().unwrap();
	//    let button = dialog.get_child("Btn0").unwrap();
	//    button.set_enabled(!button.get_enabled());
	//		                    .set_image(Image::new_rgb(ICON_WIDTH, ICON_HEIGHT, &icon_data))
	}
	
	fn set_next(dialog: Dialog) {
		let digits = game!().get_digits();
		let next_try_ = game!().get_try();
		let mut next_try = next_try_.iter();
    	for i in (0 .. digits) {
    		let d = next_try.next().unwrap().to_string();
    		let name  = format!("BtnNext_{}", i);
	 		widget_by_name!(dialog, Button, name)
			    .set_label(d);
		}
	}
	
	fn show_dialog(dialog: Dialog, titel: &str, msg: &str) -> CallbackStatus {
	    let res = AlertPopupBuilder::new(titel, msg, "Yes").button2("No (close)").button3("Cancel").popup();
        match res {
        	1 => { // yes
//        			Game::reset();
        			game!().reset();
        			Gui::set_next(dialog);
        			CallbackStatus::Default
        		},
        	3 => { // Cancel
		let digits = game!().get_digits();
			    	for i in (0 .. digits +1) {
			    		let name  = format!("BtnInput_{}", i);
				 		widget_by_name!(dialog, Button, name)
							.set_enabled(false);
					}
        			CallbackStatus::Default
        		},
        	2 => CallbackStatus::Close, // no
        	_ => unreachable!(), 
       	}
	}
}



