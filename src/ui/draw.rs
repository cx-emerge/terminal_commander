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


/// 绘制 UI
pub fn draw(f: &mut Frame<impl Backend>) {
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
	let left_file_window = FileWindow {};
	left_file_window.draw(f, file_windows_chunks[0]);

	// 右侧文件窗口
	let right_file_window = FileWindow {};
	right_file_window.draw(f, file_windows_chunks[1]);

	// 帮助文本
	let help_text = [
		Text::raw("退出: Ctrl+c"),
	];
	let help_text_widget = Paragraph::new(help_text.iter())
		.block(Block::default().title("帮助").borders(Borders::ALL))
		.style(Style::default().fg(Color::White).bg(Color::Black))
	;

	f.render_widget(help_text_widget, main_chunks[1]);
}
