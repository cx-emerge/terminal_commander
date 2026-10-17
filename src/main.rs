mod store;
mod ui;
mod components;
mod configs;


use std::{
	io,
	error,
	time,
};

use ratatui::{
	Terminal,
	backend::CrosstermBackend,
};
use crossterm;

use store::Store;

enum PromptAction {
	CreateDirectory,
	Rename,
}


fn main() -> Result<(), Box<dyn error::Error>> {
	let pattern = std::env::args().nth(1);
	if let Some(pattern) = pattern {
		if pattern == "--version" {
			println!("terminal commander {}", env!("CARGO_PKG_VERSION"));
		}

		return Ok(());
	}


	let mut store = Store::new();
	let mut prompt: Option<(PromptAction, String)> = None;

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
		terminal.draw(|mut f| match ui::draw(&mut f, &mut store, prompt.as_ref().map(|(action, name)| (match action {
			PromptAction::CreateDirectory => "新建目录: ",
			PromptAction::Rename => "重命名: ",
		}, name.as_str()))) { _ => {}, })?;

		if !crossterm::event::poll(time::Duration::from_millis(250))? {
			continue;
		}

		if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
			if key_event.kind != crossterm::event::KeyEventKind::Press {
				continue;
			}

			if let Some((action, name)) = prompt.as_mut() {
				match key_event.code {
					crossterm::event::KeyCode::Enter => {
						if !name.is_empty() {
							match action {
								PromptAction::CreateDirectory => { let _ = store.active_file_window().create_directory(name); },
								PromptAction::Rename => { let _ = store.active_file_window().rename_current(name); },
							}
						}
						prompt = None;
					},
					crossterm::event::KeyCode::Esc => prompt = None,
					crossterm::event::KeyCode::Backspace => { name.pop(); },
					crossterm::event::KeyCode::Char(ch) => name.push(ch),
					_ => {},
				}
				continue;
			}

			match (key_event.code, key_event.modifiers) {
				configs::keys::QUIT => break,

				configs::keys::CREATE_DIRECTORY => prompt = Some((PromptAction::CreateDirectory, String::new())),

				configs::keys::RENAME => prompt = Some((PromptAction::Rename, String::new())),

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
