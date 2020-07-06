use tui::{
	Frame,
	backend::Backend,
	layout::{ Rect, },
};


/// 组件特性
pub trait Component {
	fn draw(&self, f: &mut Frame<impl Backend>, area: Rect);
}
