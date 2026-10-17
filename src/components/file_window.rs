use super::traits::Component;

use std::{
	fs, error,
};

use ratatui::{
	Frame,
	layout::{ Layout, Constraint, Direction, Rect, },
	widgets::{
		Block, Borders,
		Paragraph, Wrap,
		TableState, Table, Row,
	},
	text::{ Line, Span, },
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
		f: &mut Frame<'_>,
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
		let text = vec![Line::from(Span::raw(self.dir.clone()))];
		let path_widget = Paragraph::new(text)
			.block(Block::default()
				.title(Line::from(
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
			.enumerate()
			.filter_map(|(index, dir)| {
				let file_path = dir.path();
				let Ok(file_metadata) = dir.metadata() else {
					return None;
				};

				let file_name = file_path.file_name().unwrap().to_str().unwrap();
				let file_size = format_file_size(file_metadata.len());
				let name_color = if index == self.current_index {
					Color::White
				} else if file_metadata.is_dir() {
					Color::Cyan
				} else {
					active_color
				};
				let file_modified = file_metadata.modified().ok()
					.map(|modified| DateTime::<Utc>::from(modified).format("%Y-%m-%d %H:%M:%S").to_string())
					.unwrap_or_else(|| String::from("-"));

				Some(Row::new(vec![
					Span::styled(String::from(file_name), Style::default().fg(name_color)),
					file_size.into(),
					file_modified.into(),
				]).style(Style::default().fg(active_color)))
			})
			.collect::<Vec<_>>()
		;
		let files_widget = Table::new(table_rows, table_widths)
			.block(Block::default()
				.title(Line::from(
					Span::styled("文件列表", Style::default().fg(active_color))
				))
				.borders(Borders::ALL)
				.border_style(Style::default().fg(active_color))
			)
			.header(Row::new(table_header).style(Style::default().fg(Color::Yellow)))
			.style(Style::default().fg(Color::White))
			.row_highlight_style(Style::default().bg(Color::Blue))
			.column_spacing(1)
		;
		table_state.select(Some(self.current_index));
		f.render_stateful_widget(files_widget, chunks[1], &mut table_state);

		return Ok(());
	}
}

fn format_file_size(size: u64) -> String {
	let units = ["B", "KB", "MB", "GB", "TB", "PB"];
	let mut value = size as f64;
	let mut unit = 0;
	while value >= 1024.0 && unit < units.len() - 1 {
		value /= 1024.0;
		unit += 1;
	}

	if unit == 0 {
		format!("{} {}", size, units[unit])
	} else {
		format!("{:.1} {}", value, units[unit])
	}
}
