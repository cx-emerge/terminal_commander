use std::error;

use tui::{
	Frame,
	backend::Backend,
	layout::{ Rect, },
	widgets::{ Block, Borders, Paragraph, Text, },
	style::{ Style, Color, },
};


/// 绘制帮助栏
pub fn draw_help(
	f: &mut Frame<impl Backend>,
	area: Rect
) -> Result<(), Box<dyn error::Error>> {
	let help_text = [
		Text::raw("切换窗口: Tab"),
		Text::raw(" | "),
		Text::raw("移动: j(上),k(下)"),
		Text::raw(" | "),
		Text::raw("打开: o"),
		Text::raw(" | "),
		Text::raw("返回: u"),
		Text::raw(" | "),
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

	f.render_widget(help_text_widget, area);

	return Ok(());
}
