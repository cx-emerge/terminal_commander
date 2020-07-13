use std::error;

use tui::{
	Frame,
	backend::Backend,
	layout::{ Layout, Constraint, Direction, },
};

use super::draw_help;
use super::draw_window;

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
			dir: left_file_window.dir.clone(),
			current_index: left_file_window.current_index,
		}
	)?;

	// 右侧文件窗口
	let is_active = store.active_window == 1;
	let right_file_window = store.file_window(1, 0);
	draw_window(f, windows_chunks[1],
		draw_window::FileWindowOptions{
			is_active: is_active,
			dir: right_file_window.dir.clone(),
			current_index: right_file_window.current_index,
		}
	)?;

	// 帮助文本
	draw_help(f, main_chunks[1])?;

	return Ok(());
}
