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
	filter: &str,
	focused: bool,
	search_active: bool,
) -> Result<(), Box<dyn error::Error>> {
	let border_color = if focused { Color::Yellow } else { Color::Gray };
	let command_widget = Paragraph::new(Line::from(vec![
		Span::styled(if search_active { "/" } else { "" }, Style::default().fg(Color::Yellow)),
		Span::raw(filter),
	]))
		.block(
			Block::default()
				.title(Line::from(Span::styled("命令", Style::default().fg(border_color))))
				.borders(Borders::ALL)
				.border_style(Style::default().fg(border_color)),
		);

	f.render_widget(command_widget, area);
	if focused {
		let cursor_x = area.x
			.saturating_add(2)
			.saturating_add(Span::raw(filter).width().min(u16::MAX as usize) as u16)
			.min(area.x.saturating_add(area.width.saturating_sub(2)));
		f.set_cursor_position((cursor_x, area.y.saturating_add(1)));
	}

	return Ok(());
}
