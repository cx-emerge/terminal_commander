use std::error;

use tui::{
	Frame,
	backend::Backend,
	layout::{ Layout, Constraint, Direction, },
	widgets::{ Block, Borders, Paragraph, Text, },
	style::{ Style, Color, }
};

use crate::components::{
	traits::Component,
	FileWindow,
};

use crate::store::Store;


/// 绘制 UI
pub fn draw(
	f: &mut Frame<impl Backend>,
	store: &mut Store
) -> Result<(), Box<dyn error::Error>> {
	// 主布局
	let main_chunks = Layout::default()
		.direction(Direction::Vertical)
		.margin(0)
		.constraints([
			Constraint::Min(0),
			Constraint::Length(3)
		].as_ref())
		.split(f.size())
	;

	// 文件窗口布局
	let file_windows_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.margin(0)
		.constraints([
			Constraint::Percentage(50),
			Constraint::Percentage(50),
		].as_ref())
		.split(main_chunks[0])
	;

	// 左侧文件窗口
	let left_file_window = FileWindow {
		file_selected: store.file_window(0).file_index,
	};
	left_file_window.draw(f, file_windows_chunks[0])?;

	// 右侧文件窗口
	let right_file_window = FileWindow {
		file_selected: store.file_window(1).file_index,
	};
	right_file_window.draw(f, file_windows_chunks[1])?;

	// 帮助文本
	let help_text = [
		Text::raw("退出: Ctrl+c"),
	];
	let help_text_widget = Paragraph::new(help_text.iter())
		.block(
			Block::default()
			.title("帮助")
			.title_style(Style::default().fg(Color::Gray))
			.borders(Borders::ALL)
			.border_style(Style::default().fg(Color::Gray))
		)
		.style(Style::default().fg(Color::Gray))
	;

	f.render_widget(help_text_widget, main_chunks[1]);

	Ok(())
}
