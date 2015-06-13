use game::Game;
use std::collections::HashMap;
use std::ops::Range;
use kiss_ui::button::Button;
use kiss_ui::base::BaseWidget;
use kiss_ui::container::{Horizontal, Vertical, Grid};
use kiss_ui::callback::{OnClick, CallbackStatus};
use kiss_ui::dialog::Dialog;
use kiss_ui::text::Label;
use kiss_ui::widget::Widget;
use kiss_ui::image::{Image, ImageContainer};
use kiss_ui::show_gui;
use kiss_ui::container::Orientation::Vertical as OVert;
use kiss_ui::dialog::{self, AlertPopupBuilder};

extern crate current;
use current::{ Current };
macro_rules! game (
	() => (unsafe { &mut *Current::<Game>::new() })
);

/// Get a widget by its name
///
/// downcasted and with panic msg 
///
/// # Panics
/// - `dialog` is not set
/// - there is no or more then 1 widget with this `name`
/// - widget can't be downcasted to `w_type`
///
/// # Examples
///
/// ```
///	 Label::new("Foo").set_name("Lbl_Foo"),
///	 Button::new("Button").set_name("Btn_Foo").set_onclick(btn_cb),
/// ```
/// ```
/// fn btn_cb(btn: Button) {
///	 let dialog = btn.get_dialog().expect(GPM!());
///	 let mut name = btn.get_name().expect(GPM!());
///	 name.replace("Btn", "Lbl");
///		widget_by_name!(dialog, Label, name).set_text("Bar");
/// }
/// ```
macro_rules! widget_by_name (
	($dialog:ident, $w_type:ident, $name:ident) => (
	$dialog.get_child(&$name)
		.expect(&(format!("{} not found: {}", stringify!($w_type), $name))[..])
		.try_downcast::<$w_type>().ok()
		.expect(&(format!("Widget '{}' is not a {}", $name, stringify!($w_type))[..]))
	)
);

const ICON_WIDTH: u32 = 20;
const ICON_HEIGHT: u32 = 20;
const ICON_SIZE: usize = (ICON_HEIGHT * ICON_WIDTH) as usize;

pub struct Gui;
impl<'a> Gui {
	
	pub fn new()-> Gui { Gui }

	pub fn run (&mut self) {
		show_gui(|| {
			let mut children_guess = children![Label::new("My guess: ")];
			// Add *4* Buttons with names starting with "BtnGuess"; used to display the guess
			let btns_guess = self.add_multi_btn(vec!("1", "2", "3", "4"), "Guess", &mut children_guess);
			for btn in btns_guess { btn.set_enabled(false); }
			
			let mut children_input = children![Label::new("Bulls: ").set_name("LblInput")];
			let btns_input = self.add_multi_btn(vec!("0", "1", "2", "3", "4"), "Input", &mut children_input);
			for btn in btns_input { btn.set_onclick(Gui::process_input); }
			
			// pre-generate Labels to display the output, but hide them
			// BUG: KISS-UI don't resize widgets -> set any text and images now 
			let mut children_output = children![];
			for i in (1 .. game!().get_max_turns()) {
				self.add_multi_lbl(
					vec!(&format!(" Guess #{} is ", i)[..], "-1-2-3-4-", "[x]", "[x]", "[x]", "[x]")
						, &format!("Output_{}", i)[..]
						, &mut children_output);
			}
	
			// on gtk the buttons overlap
			let spacing = if cfg!(windows) { 1 } else { 25 };
			let window = Dialog::new(
				Vertical::new(
					children![
						Horizontal::new(children_guess)
							.set_elem_spacing_pixels(spacing),
						Label::new(""), // poor IupFill() replacement
						Horizontal::new(children_input)
							.set_elem_spacing_pixels(spacing),
						Label::new(""), // Fill() replacement
						Grid::new(children_output)
							.set_ndiv(6) // BUG: don't work
							.set_orientation(OVert),
					]
				)
			)
			// env!("CARGO_PKG_NAME") isn't defined yet
			.set_title(concat!("Bulls & Cows in reverse", " v", env!("CARGO_PKG_VERSION"))) 
	//		.set_size_pixels(320, 500)
			;
			Gui::set_guess(window);
			Gui::reset_output(window);
			window
		})
	}

	fn process_input(btn: Button) -> CallbackStatus {
		// use the last digits in the name as input value
		let value: u8 =
			btn.get_name().expect("can't process on Buttons without a name")
				.rsplitn(2, "_").next().expect("Button name needs to end with '_' + digits")
				.parse::<u8>().ok().expect("Button name needs to end with '_' + digits");
		let digits = game!().get_digits();
		let dialog = btn.get_dialog().expect(GPM!());
	
		if game!().is_bulls_set() {
			// set cows
			let bulls = game!().get_bulls();
			let last_guess = game!().get_guess();
			if ! game!().set_cows(value) {
				dialog::message_popup("Warning", "Something went wrong!");
			} else {
				match game!().count() {
					0 => {
						return Gui::show_dialog(dialog, "That can't be right",
								"Please check your answers!\n\nDo you like to play again?")
						},
					
					1 => {
						return Gui::show_dialog(dialog, "I know it!",
								&format!("It is {}!\nDo you like to play again?",
									Game::arr_to_str(&game!().get_guess()) )[..])
						}
					_ => {
					},
				}
	
				Gui::set_guess(dialog);
				Gui::enable_input(dialog, (0 .. digits +1), true);
	
				let name = "LblInput";
				widget_by_name!(dialog, Label, name).set_text("Bulls: ");
				
				let turn = game!().get_turn();
				
				let icon_gray: Vec<_> = vec![(155, 155, 155); ICON_SIZE];
				let icon_red: Vec<_> = vec![(255, 0, 0); ICON_SIZE];
				let icon_green: Vec<_> = vec![(0, 255, 0); ICON_SIZE];
				let name = format!("LblOutput_{}_0", turn);
				widget_by_name!(dialog, Label, name).show();
				let name = format!("LblOutput_{}_1", turn);
				widget_by_name!(dialog, Label, name).set_text(&format!("{}: ", Game::arr_to_str(&last_guess))[..]).show();		
				for i in (2 .. digits + 2) {
					let icon = if i < (bulls +2) { &icon_green }
						else if i < ( bulls + value +2) { &icon_red }
						else { &icon_gray };
					let name = format!("LblOutput_{}_{}", turn, i);
					widget_by_name!(dialog, Label, name)
					.set_image(Image::new_rgb(ICON_WIDTH, ICON_HEIGHT, icon))
					.show();
				}
			}
		} else {
			// set bulls
			if digits == value {
				return Gui::show_dialog(dialog, "I won! :P", "Do you like to play again?")
			}
			if game!().set_bulls(value) {
				Gui::enable_input(dialog, ((digits - value +1) .. digits +1), false);
				let name = "LblInput";
				widget_by_name!(dialog, Label, name).set_text("Cows: ");
			} else {
				dialog::message_popup("Warning", "Something went wrong!");
			}
		}
		CallbackStatus::Default
	}
	
	fn add_multi_btn (&mut self, labels : Vec<&str>, base_name : &str, vec : &mut Vec<BaseWidget>) -> Vec<Button> {
		let mut buttons = Vec::<Button>::with_capacity(labels.len());
		let mut i = 0;
			for label in labels {
				let name  = format!("Btn{}_{}", base_name, i);
				let btn = Button::new()
					.set_name(&name)
					.set_label(label);
				buttons.push(btn.clone());
				vec.push(btn.to_base());
				i += 1;
			}
		buttons
	}

	fn add_multi_lbl (&mut self, labels : Vec<&str>, base_name : &str, vec : &mut Vec<BaseWidget>) {
		let mut i = 0;
			for label in labels {
				let name  = format!("Lbl{}_{}", base_name, i);
				let btn = Label::new(label).set_name(&name);
				vec.push(btn.to_base());
				i += 1;
			}
	}

	fn set_guess(dialog: Dialog) {
		let digits = game!().get_digits();
		let next_guess_ = game!().get_guess();
		let mut next_guess = next_guess_.iter();
		for i in (0 .. digits) {
			let d = next_guess.next().expect(GPM!()).to_string();
			let name  = format!("BtnGuess_{}", i);
	 		widget_by_name!(dialog, Button, name)
				.set_label(d);
		}
	}
	
	fn enable_input(dialog: Dialog, range: Range<u8>, set_to: bool) {
		for i in range {
			let name  = format!("BtnInput_{}", i);
	 		widget_by_name!(dialog, Button, name)
				.set_enabled(set_to);
		}
	}
	
	fn reset_output(dialog: Dialog) {
		let icon_gray: Vec<_> = vec![(155, 155, 155); ICON_SIZE];
			
		for i in (1 .. game!().get_max_turns()) {
			let name  = format!("LblOutput_{}_0", i);
			widget_by_name!(dialog, Label, name).hide();
			let name  = format!("LblOutput_{}_1", i);
			widget_by_name!(dialog, Label, name).hide();
			for x in (2..6u8) {
				let name  = format!("LblOutput_{}_{}", i, x);
				widget_by_name!(dialog, Label, name).hide()
					.set_image(Image::new_rgb(ICON_WIDTH, ICON_HEIGHT, &icon_gray));
			}
		}

	}
	
	fn show_dialog(dialog: Dialog, titel: &str, msg: &str) -> CallbackStatus {
		let res = AlertPopupBuilder::new(titel, msg, "Yes").button2("No (close)").button3("Cancel").popup();
		match res {
			1 => { // yes
					game!().reset();
					Gui::set_guess(dialog);
					Gui::reset_output(dialog);
					let digits = game!().get_digits();
					Gui::enable_input(dialog, (0 .. digits +1), true);
					CallbackStatus::Default
				},
			3 => { // Cancel
					let digits = game!().get_digits();
					Gui::set_guess(dialog);
					Gui::enable_input(dialog, (0 .. digits +1), false);
					CallbackStatus::Default
				},
			2 => CallbackStatus::Close, // no
			_ => unreachable!(),
		}
	}
}



