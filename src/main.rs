// File: src/main.rs
// Author: Jacob Guenther
// Date: January 2020

extern crate gtk;
extern crate roll_lang;

use std::cell::RefCell;
use std::sync::Arc;

use gtk::{ButtonExt, EntryExt};

pub mod main_window;
use main_window::MainWindow;

pub mod state;
use state::*;

pub mod roll;
use roll::roll;

static APP_NAME: &str = "gRollLang";
static OTHER_BUTTON_NAMES: [&str; 7] = [
	"roll_button",
	"undo",
	"redo",
	"clear",
	"insert_modifier",
	"insert_ndx",
	"insert_query",
];
static INSERT_OP_FN_BUTTONS: [(&str, &str); 11] = [
	("insert_operator_add", "[] + []"),
	("insert_operator_minus", "[] - []"),
	("insert_operator_multiply", "[] * []"),
	("insert_operator_divide", "[] / []"),
	("insert_operator_power", "[] ^ ([])"),
	("insert_operator_parentheses", "([])"),
	("insert_function_floor", "floor([])"),
	("insert_function_ceil", "ceil([])"),
	("insert_function_round", "round([])"),
	("insert_function_abs", "abs([])"),
	("insert_inline_roll", "[[ [] ]]"),
];

static OTHER_ENTRY_NAMES: [&str; 5] = [
	"modifier_entry",
	"ndx_n_count",
	"ndx_x_count",
	"query_prompt_entry",
	"query_default_entry",
];
static DICE_NAMES: [&str; 7] = ["d4", "d6", "d8", "d10", "d12", "d20", "d100"];

fn main() {
	if gtk::init().is_err() {
		eprintln!("Failed to initialize GTK application");
		std::process::exit(1);
	}

	let gui = Arc::new(MainWindow::default());
	let state_handler = Arc::new(RefCell::new(StateHandler::new(&State::default())));
	{
		gui.update_from(state_handler.borrow().current());
	}

	// Run the entry
	{
		let button = gui.button("roll_button");
		let gui = Arc::clone(&gui);
		let state_handler = Arc::clone(&state_handler);
		button.connect_clicked(move |_| {
			let input = gui.user_roll_entry().get_text().to_owned();
			state_handler
				.borrow_mut()
				.current_mut()
				.update_to_match_entry(gui.user_roll_entry());
			state_handler
				.borrow_mut()
				.current_mut()
				.update_from_roll_result(&roll(&input));
			gui.update_from(state_handler.borrow().current());
		});
	}

	{
		let user_roll_entry = gui.user_roll_entry();
		let gui = Arc::clone(&gui);
		let state_handler = Arc::clone(&state_handler);
		user_roll_entry.connect_activate(move |entry| {
			let input = entry.get_text().to_owned();
			state_handler
				.borrow_mut()
				.current_mut()
				.update_to_match_entry(gui.user_roll_entry());
			state_handler
				.borrow_mut()
				.current_mut()
				.update_from_roll_result(&roll(&input));
			gui.update_from(state_handler.borrow().current());
		});
	}

	// The clear, undo, and redo commands
	{
		let button = gui.button("clear");
		let gui = Arc::clone(&gui);
		let state_handler = Arc::clone(&state_handler);
		button.connect_clicked(move |_| {
			state_handler
				.borrow_mut()
				.execute(&mut Box::new(ClearCommand::default()));
			gui.update_from(state_handler.borrow().current());
		});
	}

	{
		let button = gui.button("undo");
		let gui = Arc::clone(&gui);
		let state_handler = Arc::clone(&state_handler);
		button.connect_clicked(move |_| {
			state_handler.borrow_mut().undo();
			gui.update_from(state_handler.borrow().current());
		});
	}

	{
		let button = gui.button("redo");
		let gui = Arc::clone(&gui);
		let state_handler = Arc::clone(&state_handler);
		button.connect_clicked(move |_| {
			state_handler.borrow_mut().redo();
			gui.update_from(state_handler.borrow().current());
		});
	}

	// Insert operators, functions, and inline rolls
	for &(name, what_to_insert) in INSERT_OP_FN_BUTTONS.iter() {
		let button = gui.button(name);
		let gui = Arc::clone(&gui);
		let state_handler = Arc::clone(&state_handler);
		button.connect_clicked(move |_| {
			state_handler
				.borrow_mut()
				.current_mut()
				.update_to_match_entry(gui.user_roll_entry());
			state_handler
				.borrow_mut()
				.execute(&mut Box::new(InsertCommand::new(what_to_insert)));
			gui.update_from(state_handler.borrow().current());
		});
	}

	// Insert a modifier
	{
		let button = gui.button("insert_modifier");
		let gui = Arc::clone(&gui);
		let state_handler = Arc::clone(&state_handler);
		button.connect_clicked(move |_| {
			let modifier = gui.entry("modifier_entry").get_text().to_owned();
			if modifier.parse::<i32>().is_err() && modifier.parse::<f32>().is_err() {
				return;
			};
			state_handler
				.borrow_mut()
				.current_mut()
				.update_to_match_entry(gui.user_roll_entry());
			state_handler
				.borrow_mut()
				.execute(&mut Box::new(InsertCommand::new(&modifier)));
			gui.update_from(state_handler.borrow().current());
		});
	}

	// Insert a normal dice
	for &name in DICE_NAMES.iter() {
		let button = gui.button(&format!("insert_{}", name));
		let gui = Arc::clone(&gui);
		let state_handler = Arc::clone(&state_handler);
		button.connect_clicked(move |_| {
			let count = gui.entry(&format!("{}_count", name)).get_text().to_owned();
			if count.parse::<u32>().is_err() && count.parse::<f32>().is_err() {
				return;
			};
			state_handler
				.borrow_mut()
				.current_mut()
				.update_to_match_entry(gui.user_roll_entry());
			state_handler
				.borrow_mut()
				.execute(&mut Box::new(InsertCommand::new(&format!(
					"{}{}",
					count, name,
				))));
			gui.update_from(state_handler.borrow().current());
		});
	}

	// Insert a custom dice
	{
		let button = gui.button("insert_ndx");
		let gui = Arc::clone(&gui);
		let state_handler = Arc::clone(&state_handler);
		button.connect_clicked(move |_| {
			let count = gui.entry("ndx_n_count").get_text().to_owned();
			if count.parse::<u32>().is_err() && count.parse::<f32>().is_err() {
				return;
			};
			let sides = gui.entry("ndx_x_count").get_text().to_owned();
			if sides.parse::<u32>().is_err() {
				return;
			};
			state_handler
				.borrow_mut()
				.current_mut()
				.update_to_match_entry(gui.user_roll_entry());
			state_handler
				.borrow_mut()
				.execute(&mut Box::new(InsertCommand::new(&format!(
					"{}d{}",
					count, sides,
				))));
			gui.update_from(state_handler.borrow().current());
		});
	}

	// Insert a query
	{
		let button = gui.button("insert_query");
		let gui = Arc::clone(&gui);
		let state_handler = Arc::clone(&state_handler);
		button.connect_clicked(move |_| {
			let prompt = gui.entry("query_prompt_entry").get_text().to_owned();
			let default = gui.entry("query_default_entry").get_text().to_owned();
			state_handler
				.borrow_mut()
				.current_mut()
				.update_to_match_entry(gui.user_roll_entry());
			state_handler
				.borrow_mut()
				.execute(&mut Box::new(InsertCommand::new(&format!(
					"?{{{} | {} }}",
					prompt, default,
				))));
			gui.update_from(state_handler.borrow().current());
		});
	}

	gui.start();
	gtk::main();
}
