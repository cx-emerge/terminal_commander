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
	Delete,
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
	let mut search_focused = false;
	let mut search_active = false;

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
		let prompt_view = prompt.as_ref().map(|(action, name)| {
			let label = match action {
				PromptAction::CreateDirectory => "新建目录: ",
				PromptAction::Rename => "重命名: ",
				PromptAction::Delete => "删除所选项？(y/n): ",
			};
			(label, name.as_str())
		});
		terminal.draw(|mut f| match ui::draw(&mut f, &mut store, prompt_view, search_focused, search_active) { _ => {}, })?;

		if !crossterm::event::poll(time::Duration::from_millis(250))? {
			continue;
		}

		if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
			if key_event.kind != crossterm::event::KeyEventKind::Press {
				continue;
			}

			if search_focused {
				match key_event.code {
					crossterm::event::KeyCode::Enter => search_focused = false,
					crossterm::event::KeyCode::Esc => {
						store.active_file_window().set_filter("");
						search_focused = false;
						search_active = false;
					},
					crossterm::event::KeyCode::Backspace => {
						let mut filter = store.active_file_window().filter.clone();
						filter.pop();
						store.active_file_window().set_filter(&filter);
					},
					crossterm::event::KeyCode::Char(ch) if key_event.modifiers == crossterm::event::KeyModifiers::NONE => {
						let mut filter = store.active_file_window().filter.clone();
						filter.push(ch);
						store.active_file_window().set_filter(&filter);
					},
					_ => {},
				}
				continue;
			}

			if let Some((action, name)) = prompt.as_mut() {
				if matches!(action, PromptAction::Delete) {
					match key_event.code {
						crossterm::event::KeyCode::Char('y') | crossterm::event::KeyCode::Char('Y') => {
							let _ = store.active_file_window().delete_current();
							prompt = None;
						},
						crossterm::event::KeyCode::Char('n') | crossterm::event::KeyCode::Char('N') | crossterm::event::KeyCode::Esc => prompt = None,
						_ => {},
					}
					continue;
				}

				match key_event.code {
					crossterm::event::KeyCode::Enter => {
						if !name.is_empty() {
							match action {
								PromptAction::CreateDirectory => { let _ = store.active_file_window().create_directory(name); },
								PromptAction::Rename => { let _ = store.active_file_window().rename_current(name); },
								PromptAction::Delete => {},
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

				configs::keys::SEARCH => {
					search_focused = true;
					search_active = true;
				},

				(crossterm::event::KeyCode::Esc, crossterm::event::KeyModifiers::NONE) if search_active => {
					store.active_file_window().set_filter("");
					search_active = false;
				},

				configs::keys::CREATE_DIRECTORY => prompt = Some((PromptAction::CreateDirectory, String::new())),

				configs::keys::RENAME => prompt = Some((PromptAction::Rename, String::new())),

				configs::keys::DELETE => prompt = Some((PromptAction::Delete, String::new())),

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
					let file_window = store.active_file_window();
					let previous_dir = file_window.dir.clone();
					file_window.open();
					if file_window.dir != previous_dir {
						file_window.set_filter("");
						search_focused = false;
						search_active = false;
					}
				},

				configs::keys::BACK_TO_THE_PARENT_DIRECTORY => {
					let file_window = store.active_file_window();
					let previous_dir = file_window.dir.clone();
					file_window.up();
					if file_window.dir != previous_dir {
						file_window.set_filter("");
						search_focused = false;
						search_active = false;
					}
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
