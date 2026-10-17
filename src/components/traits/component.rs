use std::error;

use ratatui::{
	Frame,
	layout::{ Rect, },
};


/// 组件特性
pub trait Component {
	fn draw(
		&self,
		f: &mut Frame<'_>,
		area: Rect
	) -> Result<(), Box<dyn error::Error>>;
}
