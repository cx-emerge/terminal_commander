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


fn main() -> Result<(), Box<dyn error::Error>> {
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
		terminal.draw(|mut f| ui::draw(&mut f))?;

		if !crossterm::event::poll(time::Duration::from_millis(250))? {
			continue;
		}

		if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
			match key_event {
				crossterm::event::KeyEvent {
					code: crossterm::event::KeyCode::Char('c'),
					modifiers: crossterm::event::KeyModifiers::CONTROL
				} => break,

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
