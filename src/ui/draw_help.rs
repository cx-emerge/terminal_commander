use std::error;

use ratatui::{
	Frame,
	layout::{ Rect, },
	widgets::{ Block, Borders, Paragraph, },
	text::{ Line, Span },
	style::{ Style, Color, },
};


/// 绘制帮助栏
pub fn draw_help(
		f: &mut Frame<'_>,
	area: Rect
) -> Result<(), Box<dyn error::Error>> {
	let help_text = Line::from(vec![
		Span::raw("切换窗口: Tab"),
		Span::raw(" | "),
		Span::raw("移动: j(上),k(下)"),
		Span::raw(" | "),
		Span::raw("打开: o"),
		Span::raw(" | "),
		Span::raw("返回: u"),
		Span::raw(" | "),
		Span::raw("新建目录: F7"),
		Span::raw(" | "),
		Span::raw("新建文件: Shift+F4"),
		Span::raw(" | "),
		Span::raw("复制: F5"),
		Span::raw(" | "),
		Span::raw("刷新: Shift+R"),
		Span::raw(" | "),
		Span::raw("重命名: F2"),
		Span::raw(" | "),
		Span::raw("删除: Delete"),
		Span::raw(" | "),
		Span::raw("过滤: /"),
		Span::raw(" | "),
		Span::raw("退出: Ctrl+c"),
	]);

	let help_text_widget = Paragraph::new(help_text)
		.block(
			Block::default()
			.title(Line::from(
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
