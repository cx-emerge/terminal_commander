use super::traits::Component;

use std::{
	fs, error,
};

use tui::{
	Frame,
	backend::Backend,
	layout::{ Layout, Constraint, Direction, Rect, },
	widgets::{
		Block, Borders,
		Paragraph, Text,
		TableState, Table, Row,
	},
	style::{ Style, Color, },
};

use chrono::{
	DateTime, Utc
};


/// 文窗口
pub struct FileWindow {
	/// 目录路径
	pub dir: String,

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
				Constraint::Min(0),
			].as_ref())
			.split(area)
		;

		// 路径
		let text = [Text::raw(self.dir.clone())];
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
		f.render_widget(path_widget, chunks[0]);

		// 文件列表
		let mut table_state = TableState::default();
		let size_item_width = 10;
		let created_item_width = 23;
		let name_item_width = match
			area.width > (size_item_width + created_item_width)
		{
			true => area.width - (size_item_width + created_item_width),
			false => 0,
		};
		let table_widths = [
			Constraint::Length(name_item_width),
			Constraint::Length(size_item_width),
			Constraint::Length(created_item_width),
		];
		let table_header = [ "文件名", "文件大小", "修改时间", ];
		let mut table_rows = fs::read_dir(self.dir.clone())?
			.map(|entry| entry.unwrap())
			.collect::<Vec<_>>()
		;
		table_rows.sort_by_key(|dir| dir.path());
		let table_rows = table_rows.iter()
			.map(|dir| {
				let file_path = dir.path();
				let file_metadata = dir.metadata().unwrap();

				let file_name = file_path.file_name().unwrap().to_str().unwrap();
				let file_size = file_metadata.len();
				let file_created = DateTime::<Utc>::from(
					file_metadata.created().unwrap()
				).format("%Y-%m-%d %H:%M:%S");

				return Row::Data(vec![
					String::from(file_name),
					file_size.to_string(),
					file_created.to_string(),
				].into_iter());
			})
			.collect::<Vec<_>>()
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
		f.render_stateful_widget(files_widget, chunks[1], &mut table_state);

		return Ok(());
	}
}
