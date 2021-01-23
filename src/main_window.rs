use crate::gtk::prelude::*;
use gtk::*;

use std::collections::HashMap;

use super::State;
use super::{APP_NAME, DICE_NAMES, INSERT_OP_FN_BUTTONS, OTHER_BUTTON_NAMES, OTHER_ENTRY_NAMES};

pub struct MainWindow {
	window: gtk::Window,
	result: gtk::Label,
	user_roll_entry: gtk::Entry,
	entries: HashMap<String, gtk::Entry>,
	buttons: HashMap<String, gtk::Button>,
}
impl Default for MainWindow {
	fn default() -> Self {
		let glade_src = include_str!("../data/layout.glade");
		let builder = gtk::Builder::from_string(glade_src);

		let window: Window = builder
			.get_object("main_window")
			.expect("Could not get window main_window.");
		window.set_title(APP_NAME);

		let result = builder
			.get_object("roll_result")
			.expect("Could not get label roll_result");

		let user_roll_entry = builder
			.get_object("user_roll_entry")
			.expect("Could not get entry user_roll_entry.");

		let buttons = INSERT_OP_FN_BUTTONS
			.iter()
			.map(|&(name, _)| name.to_owned())
			.chain(OTHER_BUTTON_NAMES.iter().map(|&name| name.to_owned()))
			.chain(DICE_NAMES.iter().map(|d| format!("insert_{}", d)))
			.map(|name: String| {
				(
					name.clone(),
					builder
						.get_object(&name)
						.unwrap_or_else(|| panic!("Could not get button: {}", name)),
				)
			})
			.collect();

		let entries = DICE_NAMES
			.iter()
			.map(|&dice_name| format!("{}_count", dice_name))
			.chain(
				OTHER_ENTRY_NAMES
					.iter()
					.map(|&entry_name| entry_name.to_owned()),
			)
			.map(|name| {
				let entry = builder
					.get_object(&name)
					.unwrap_or_else(|| panic!("Could not get entry {}", &name));
				(name, entry)
			})
			.collect();

		MainWindow {
			window,
			result,
			user_roll_entry,
			entries,
			buttons,
		}
	}
}
impl MainWindow {
	pub fn start(&self) {
		self.window.connect_delete_event(|_, _| {
			gtk::main_quit();
			Inhibit(false)
		});
		self.window.show_all();
	}

	pub fn update_from(&self, state: &State) {
		let val = match state.roll_result() {
			Ok(ref val) => val,
			Err(ref err) => err,
		};
		self.result.set_text(val);
		self.user_roll_entry.set_text(&state.roll_entry);
		self.user_roll_entry
			.set_position(state.roll_entry_cursor as i32);
		self.user_roll_entry.grab_focus_without_selecting();
	}

	pub fn button(&self, name: &str) -> &gtk::Button {
		self.buttons
			.get(name)
			.unwrap_or_else(|| panic!("Could not get button: {}.", name))
	}
	pub fn entry(&self, name: &str) -> &gtk::Entry {
		self.entries
			.get(name)
			.unwrap_or_else(|| panic!("Could not get entry {}", name))
	}
	pub fn user_roll_entry(&self) -> &gtk::Entry {
		&self.user_roll_entry
	}
}
