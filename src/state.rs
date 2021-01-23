use gtk::{EditableExt, EntryExt};
use std::collections::VecDeque;

pub type RollResult = Result<String, String>;

#[derive(Debug, Clone)]
pub struct State {
	pub roll_entry_cursor: usize,
	pub roll_entry: String,
	roll_result: RollResult,
}
impl Default for State {
	fn default() -> Self {
		State {
			roll_entry_cursor: 4,
			roll_entry: String::from("/r [] \\"),
			roll_result: Ok(String::new()),
		}
	}
}
impl State {
	pub fn update_to_match_entry(&mut self, entry: &gtk::Entry) {
		self.roll_entry_cursor = entry.get_position() as usize;
		self.roll_entry = entry.get_text().to_string();
	}
	pub fn update_from_roll_result(&mut self, res: &RollResult) {
		self.roll_result = res.clone();
	}
	pub fn roll_result(&self) -> &RollResult {
		&self.roll_result
	}
	pub fn execute(&mut self, command: &mut Box<impl Command>) {
		*self = command.execute(self);
	}

	pub fn find_next_insert_pos(start: usize, s: &str) -> Option<usize> {
		for (i, (c1, c2)) in s
			.chars()
			.skip(start - 1)
			.zip(s.chars().skip(start))
			.enumerate()
		{
			if c1 == '[' && c2 == ']' {
				return Some(start + i);
			}
		}

		None
	}
}

pub struct StateHandler {
	previous_state_stack: VecDeque<State>,
	current: State,
	future_state_stack: VecDeque<State>,
}
impl StateHandler {
	pub fn new(current: &State) -> StateHandler {
		StateHandler {
			previous_state_stack: VecDeque::with_capacity(16),
			current: current.clone(),
			future_state_stack: VecDeque::with_capacity(16),
		}
	}
	pub fn current(&self) -> &State {
		&self.current
	}
	pub fn current_mut(&mut self) -> &mut State {
		&mut self.current
	}
	pub fn undo(&mut self) -> &State {
		if let Some(previous) = self.previous_state_stack.pop_back() {
			self.future_state_stack.push_front(self.current.clone());
			self.current = previous;
			&self.current
		} else {
			&self.current
		}
	}
	pub fn redo(&mut self) -> &State {
		if let Some(next) = self.future_state_stack.pop_front() {
			self.previous_state_stack.push_back(self.current.clone());
			self.current = next;
			&self.current
		} else {
			&self.current
		}
	}
	pub fn execute(&mut self, command: &mut Box<impl Command>) -> &State {
		let next = command.execute(&self.current);
		self.previous_state_stack.push_back(self.current.clone());
		self.current = next;
		&self.current
	}
}

pub trait Command
where
	Self: Sized,
{
	fn execute(&mut self, state: &State) -> State;
}

pub struct InsertCommand<'a> {
	s: &'a str,
}
impl<'a> InsertCommand<'a> {
	pub fn new(s: &'a str) -> InsertCommand {
		InsertCommand { s }
	}
}
impl<'a> Command for InsertCommand<'a> {
	fn execute(&mut self, state: &State) -> State {
		let mut chars = state.roll_entry.chars();
		let c1 = chars.nth(state.roll_entry_cursor - 1);
		let c2 = chars.next();
		if Some('[') == c1 && Some(']') == c2 {
			let new_entry_val = format!(
				"{}{}{}",
				&state.roll_entry[0..(state.roll_entry_cursor - 1)],
				self.s,
				&state.roll_entry[(state.roll_entry_cursor + 1)..]
			);
			let new_cursor_pos =
				State::find_next_insert_pos(state.roll_entry_cursor, &new_entry_val).unwrap_or_else(||
					State::find_next_insert_pos(1, &new_entry_val).unwrap_or_else(|| new_entry_val.len()),
				);
			State {
				roll_entry_cursor: new_cursor_pos,
				roll_entry: new_entry_val,
				roll_result: state.roll_result.clone(),
			}
		} else {
			state.clone()
		}
	}
}
pub struct ClearCommand {}
impl Default for ClearCommand {
    fn default() -> ClearCommand {
        ClearCommand {}
    }
}
impl Command for ClearCommand {
	fn execute(&mut self, _state: &State) -> State {
		State::default()
	}
}
