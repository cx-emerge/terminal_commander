use std::error;

use ratatui::{
	Frame,
	layout::Rect,
	widgets::{Block, Borders, Paragraph},
	text::{Line, Span},
	style::{Color, Style},
};

/// 绘制命令栏
pub fn draw_command(
		f: &mut Frame<'_>,
	area: Rect,
) -> Result<(), Box<dyn error::Error>> {
	let command_widget = Paragraph::new(Line::from(Span::raw("")))
		.block(
			Block::default()
				.title(Line::from(Span::styled("命令", Style::default().fg(Color::Gray))))
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Gray)),
		);

	f.render_widget(command_widget, area);

	return Ok(());
}
