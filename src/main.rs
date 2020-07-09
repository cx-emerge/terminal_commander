mod store;
mod ui;
mod components;


use std::{
	io, io::Write,
	error,
	time,
};

use tui::{
	Terminal,
	backend::CrosstermBackend,
};
use crossterm;

use store::Store;


fn main() -> Result<(), Box<dyn error::Error>> {
	let mut store = Store::new();

	crossterm::terminal::enable_raw_mode()?;

	let mut stdout = io::stdout();
	crossterm::execute!(
		stdout,
		crossterm::terminal::EnterAlternateScreen
	)?;

	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	terminal.hide_cursor()?;
	terminal.clear()?;

	loop {
		terminal.draw(|mut f| match ui::draw(&mut f, &mut store) { _ => {}, })?;

		if !crossterm::event::poll(time::Duration::from_millis(250))? {
			continue;
		}

		if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
			match key_event {
				crossterm::event::KeyEvent {
					code: crossterm::event::KeyCode::Char('c'),
					modifiers: crossterm::event::KeyModifiers::CONTROL,
				} => break,

				crossterm::event::KeyEvent {
					code: crossterm::event::KeyCode::Tab,
					modifiers: crossterm::event::KeyModifiers::NONE,
				} => {
					store.toggle_window();
				},

				crossterm::event::KeyEvent {
					code: crossterm::event::KeyCode::Char('j'),
					modifiers: crossterm::event::KeyModifiers::NONE,
				} => {
					store.active_file_window().next();
				},

				crossterm::event::KeyEvent {
					code: crossterm::event::KeyCode::Char('k'),
					modifiers: crossterm::event::KeyModifiers::NONE,
				} => {
					store.active_file_window().prev();
				},

				crossterm::event::KeyEvent {
					code: crossterm::event::KeyCode::Char('g'),
					modifiers: crossterm::event::KeyModifiers::NONE,
				} => {
					store.active_file_window().back_to_top();
				},

				_ => {}
			}
		}
	}

	terminal.clear()?;
	crossterm::terminal::disable_raw_mode()?;
	crossterm::execute!(
		terminal.backend_mut(),
		crossterm::terminal::LeaveAlternateScreen
	)?;
	terminal.show_cursor()?;

	Ok(())
}
