use std::error;

use tui::{
	Frame,
	backend::Backend,
	layout::{ Rect, },
	widgets::{ Block, Borders, Paragraph, },
	text::{ Spans, Span },
	style::{ Style, Color, },
};


/// 绘制帮助栏
pub fn draw_help(
	f: &mut Frame<impl Backend>,
	area: Rect
) -> Result<(), Box<dyn error::Error>> {
	let help_text = Spans::from(vec![
		Span::raw("切换窗口: Tab"),
		Span::raw(" | "),
		Span::raw("移动: j(上),k(下)"),
		Span::raw(" | "),
		Span::raw("打开: o"),
		Span::raw(" | "),
		Span::raw("返回: u"),
		Span::raw(" | "),
		Span::raw("退出: Ctrl+c"),
	]);

	let help_text_widget = Paragraph::new(help_text)
		.block(
			Block::default()
			.title(Spans::from(
				Span::styled("帮助", Style::default().fg(Color::Gray))
			))
			.borders(Borders::ALL)
			.border_style(Style::default().fg(Color::Gray))
		)
		.style(Style::default().fg(Color::Gray))
	;

	f.render_widget(help_text_widget, area);

	return Ok(());
}
