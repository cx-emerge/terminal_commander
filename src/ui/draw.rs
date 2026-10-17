use std::error;

use ratatui::{
	Frame,
	layout::{ Layout, Constraint, Direction, Rect, },
	widgets::{ Block, Borders, Clear, Paragraph },
	style::{ Color, Style },
	text::{ Line, Span },
};

use super::draw_help;
use super::draw_command;
use super::draw_window;

use crate::store::Store;


/// 绘制 UI
pub fn draw(
		f: &mut Frame<'_>,
	store: &mut Store,
	prompt: Option<(&str, &str)>,
) -> Result<(), Box<dyn error::Error>> {
	// 主布局
	let main_chunks = Layout::default()
		.direction(Direction::Vertical)
		.margin(0)
		.constraints([
			Constraint::Min(0),
			Constraint::Length(3),
			Constraint::Length(3)
		].as_ref())
		.split(f.area())
	;

	// 窗口布局
	let windows_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.margin(0)
		.constraints([
			Constraint::Percentage(50),
			Constraint::Percentage(50),
		].as_ref())
		.split(main_chunks[0])
	;

	// 左侧文件窗口
	let is_active = store.active_window == 0;
	let left_file_window = store.file_window(0, 0);
	draw_window(f, windows_chunks[0],
		draw_window::FileWindowOptions{
			is_active: is_active,
			dir: &left_file_window.dir,
			current_index: left_file_window.current_index,
			list: &left_file_window.list,
		}
	)?;

	// 右侧文件窗口
	let is_active = store.active_window == 1;
	let right_file_window = store.file_window(1, 0);
	draw_window(f, windows_chunks[1],
		draw_window::FileWindowOptions{
			is_active: is_active,
			dir: &right_file_window.dir,
			current_index: right_file_window.current_index,
			list: &right_file_window.list,
		}
	)?;

	// 命令栏
	draw_command(f, main_chunks[1])?;

	// 帮助文本
	draw_help(f, main_chunks[2])?;

	if let Some((label, name)) = prompt {
		let area = f.area();
		let prompt = Rect::new(area.width / 4, area.height / 2, area.width / 2, 3);
		f.render_widget(Clear, prompt);
		f.render_widget(
			Paragraph::new(Line::from(vec![Span::raw(label), Span::raw(name)]))
				.block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow))),
			prompt,
		);
	}

	return Ok(());
}
