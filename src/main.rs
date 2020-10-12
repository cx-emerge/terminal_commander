mod store;
mod ui;
mod components;
mod configs;


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
	let pattern = std::env::args().nth(1);
	if let Some(pattern) = pattern {
		if pattern == "--version" {
			println!("terminal commander {}", env!("CARGO_PKG_VERSION"));
		}

		return Ok(());
	}


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
				configs::keys::QUIT => break,

				configs::keys::TOGGLE_WINDOW => {
					store.toggle_window();
				},

				configs::keys::FILES_CURSOR_NEXT => {
					store.active_file_window().next();
				},

				configs::keys::FILES_CURSOR_PREV => {
					store.active_file_window().prev();
				},

				configs::keys::FILES_CURSOR_TOP => {
					store.active_file_window().back_to_top();
				},

				configs::keys::OPEN_DIR => {
					store.active_file_window().open();
				},

				configs::keys::BACK_TO_THE_PARENT_DIRECTORY => {
					store.active_file_window().up();
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
