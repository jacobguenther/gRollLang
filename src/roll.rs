// File: src/roll.rs
// Author: Jacob Guenther
// Date: January 2020

use std::sync::Arc;

use crate::gtk::prelude::*;
use gtk::*;

use roll_lang::interpreter::*;

pub fn roll(input: &str) -> Result<String, String> {
	let out = roll_lang::InterpreterBuilder::default()
		.with_source(input)
		.with_query_prompter(RollQueryPopup::create_popup)
		.build()
		.interpret();
	match out.error {
		Some(e) => Err(format!("{:?}", e)),
		None => Ok(out.to_string()),
	}
}

pub struct RollQueryPopup {
	dialog: MessageDialog,
	entry: Entry,
}
impl<'a, 'b> RollQueryPopup {
	pub fn create_popup(prompt: &str, default: &str) -> Option<String> {
		let popup = Arc::new(RollQueryPopup::new(Some(prompt), Some(default)));
		{
			let entry = &popup.entry;
			let popup = Arc::clone(&popup);
			entry.connect_activate(move |_entry| {
				popup.dialog.response(ResponseType::Ok);
			});
		}
		popup.run()
	}
	pub fn new(prompt: Option<&'a str>, default: Option<&'b str>) -> RollQueryPopup {
		let glade_src = include_str!("../data/prompt.glade");
		let builder = gtk::Builder::from_string(glade_src);

		let dialog: MessageDialog = builder
			.get_object("prompt_dialog_popup")
			.expect("Could not get window prompt_dialog_popup.");

		dialog.set_property_text(prompt);

		let entry: Entry = builder
			.get_object("prompt_entry")
			.expect("Could not get entry prompt_entry");
		entry.set_text(&default.unwrap());

		dialog.add_button("Enter", ResponseType::Ok);

		RollQueryPopup { dialog, entry }
	}
	pub fn run(&self) -> Option<String> {
		let text = if self.dialog.run() == ResponseType::Ok {
			Some(self.entry.get_text().to_string())
		} else {
			None
		};
		unsafe {
			self.dialog.destroy();
		}
		text
	}
}
