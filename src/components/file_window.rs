use super::traits::Component;

use std::{
	io, fs, error,
};

use tui::{
	Frame,
	backend::Backend,
	layout::{ Layout, Constraint, Direction, Rect, },
	widgets::{
		Block, Borders,
		Tabs, Paragraph, Text,
		TableState, Table, Row,
	},
	style::{ Style, Color, },
};
use dirs;
use chrono::{
	DateTime, Utc
};


/// 文窗口
pub struct FileWindow {
	/// 当前选的文件
	pub file_selected: usize,
}

impl Component for FileWindow {
	fn draw(
		&self,
		f: &mut Frame<impl Backend>,
		area: Rect
	) -> Result<(), Box<dyn error::Error>> {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(0)
			.constraints([
				Constraint::Length(3),
				Constraint::Length(3),
				Constraint::Min(0),
			].as_ref())
			.split(area)
		;

		// 标签
		let tabs_widget = Tabs::default()
			.block(Block::default()
				.title("窗口标签")
				.title_style(Style::default().fg(Color::Gray))
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Gray))
			)
			.titles(&["projects"])
			.style(Style::default().fg(Color::White))
			.highlight_style(Style::default().fg(Color::Yellow))
			.divider("|")
		;
		f.render_widget(tabs_widget, chunks[0]);

		// 路径
		let text = [Text::raw("~")];
		let path_widget = Paragraph::new(text.iter())
			.block(Block::default()
				.title("路径")
				.title_style(Style::default().fg(Color::Gray))
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Gray))
			)
			.style(Style::default().fg(Color::Gray))
			.wrap(true)
		;
		f.render_widget(path_widget, chunks[1]);

		// 文件列表
		let mut table_state = TableState::default();
		let table_widths = [
			Constraint::Length(area.width - 10 - 23),
			Constraint::Length(10),
			Constraint::Length(23),
		];
		let table_header = [ "文件名", "文件大小", "修改时间", ];
		let home_dir = match dirs::home_dir() {
			Some(home_dir) => match home_dir.to_str() {
				Some(home_dir) => home_dir.to_string(),
				_ => ".".to_string(),
			},
			_ => ".".to_string(),
		};
		let table_rows = fs::read_dir(home_dir)?
			.map(|res| {
				res.map(|entry| {
					let file_path = entry.path();
					let file_metadata = entry.metadata().unwrap();

					let file_name = file_path.file_name().unwrap().to_str().unwrap();
					let file_size = file_metadata.len();
					let file_created = DateTime::<Utc>::from(
						file_metadata.created().unwrap()
					).format("%Y-%m-%d %H:%M:%S");

					Row::Data(vec![
						String::from(file_name),
						file_size.to_string(),
						file_created.to_string()
					].into_iter())
				})
			})
			.collect::<Result<Vec<_>, io::Error>>()?
		;
		let files_widget = Table::new(
				table_header.iter(),
				table_rows.into_iter()
			)
			.block(Block::default().title("文件列表").borders(Borders::ALL))
			.header_style(Style::default().fg(Color::Yellow))
			.widths(&table_widths)
			.style(Style::default().fg(Color::White))
			.highlight_style(Style::default().bg(Color::Blue))
			.column_spacing(1)
		;
		table_state.select(Some(self.file_selected));
		f.render_stateful_widget(files_widget, chunks[2], &mut table_state);

		Ok(())
	}
}
