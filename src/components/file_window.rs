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
		Paragraph, Wrap,
		TableState, Table, Row,
	},
	text::{ Spans, Span, },
	style::{ Style, Color, },
};

use chrono::{
	DateTime, Utc
};


/// 文件窗口
pub struct FileWindow<'a> {
	/// 是否为激活的
	pub is_active: bool,

	/// 目录路径
	pub dir: &'a String,

	/// 当前文件索引
	pub current_index: usize,

	/// 列表
	pub list: &'a Vec<fs::DirEntry>,
}

impl Component for FileWindow<'_> {
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
		let text = vec![
			Spans::from(Span::raw(self.dir.clone()))
		];
		let path_widget = Paragraph::new(text)
			.block(Block::default()
				.title(Spans::from(
					Span::styled("路径", Style::default().fg(Color::Gray))
				))
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Gray))
			)
			.style(Style::default().fg(Color::Gray))
			.wrap(Wrap { trim: true })
		;
		f.render_widget(path_widget, chunks[0]);

		// 文件列表
		let active_color = if self.is_active
			{ Color::White } else { Color::Gray }
		;
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
		let table_rows = self.list.iter()
			.map(|dir| {
				let file_path = dir.path();
				let file_metadata = dir.metadata().unwrap();

				let file_name = file_path.file_name().unwrap().to_str().unwrap();
				let file_size = file_metadata.len();
				let file_created = DateTime::<Utc>::from(
					file_metadata.created().unwrap()
				).format("%Y-%m-%d %H:%M:%S");

				return Row::StyledData(vec![
					String::from(file_name),
					file_size.to_string(),
					file_created.to_string(),
				].into_iter(), Style::default().fg(active_color));
			})
			.collect::<Vec<_>>()
		;
		let files_widget = Table::new(
				table_header.iter(),
				table_rows.into_iter()
			)
			.block(Block::default()
				.title(Spans::from(
					Span::styled("文件列表", Style::default().fg(active_color))
				))
				.borders(Borders::ALL)
				.border_style(Style::default().fg(active_color))
			)
			.header_style(Style::default().fg(Color::Yellow))
			.widths(&table_widths)
			.style(Style::default().fg(Color::White))
			.highlight_style(Style::default().bg(Color::Blue))
			.column_spacing(1)
		;
		table_state.select(Some(self.current_index));
		f.render_stateful_widget(files_widget, chunks[1], &mut table_state);

		return Ok(());
	}
}
