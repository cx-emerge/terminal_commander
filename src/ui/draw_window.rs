use std::{
	fs, error,
};

use tui::{
	Frame,
	backend::Backend,
	layout::{ Layout, Constraint, Direction, Rect, },
	widgets::{ Block, Borders, Tabs, },
	text::{ Spans, Span, },
	style::{ Style, Color, },
};

use crate::components::{
	traits::Component,
	FileWindow,
};


pub struct FileWindowOptions<'a> {
	pub is_active: bool,

	pub dir: &'a String,

	pub current_index: usize,

	pub list: &'a Vec<fs::DirEntry>,
}


/// 绘制窗口
pub fn draw_window(
	f: &mut Frame<impl Backend>,
	area: Rect,
	file_window_options: FileWindowOptions
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

	// 标签
	let tabs_widget = Tabs::new(
		["projects"].iter().cloned().map(Spans::from).collect()
	)
		.block(Block::default()
			.title(Spans::from(
				Span::styled("窗口标签", Style::default().fg(Color::Gray))
			))
			.borders(Borders::ALL)
			.border_style(Style::default().fg(Color::Gray))
		)
		.style(Style::default().fg(Color::White))
		.highlight_style(Style::default().fg(Color::Yellow))
		.divider("|")
	;
	f.render_widget(tabs_widget, chunks[0]);

	// 文件窗口
	let file_window_widget = FileWindow {
		is_active: file_window_options.is_active,
		dir: file_window_options.dir,
		current_index: file_window_options.current_index,
		list: file_window_options.list,
	};
	file_window_widget.draw(f, chunks[1])?;

	return Ok(());
}
