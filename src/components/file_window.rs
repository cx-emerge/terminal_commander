use super::traits::Component;

use tui::{
	Frame,
	backend::Backend,
	layout::{ Layout, Constraint, Direction, Rect, },
	widgets::{ Block, Borders, Tabs, Paragraph, Text, Table, Row, },
	style::{ Style, Color, },
};


/// 文窗口
pub struct FileWindow {

}

impl Component for FileWindow {
	fn draw(&self, f: &mut Frame<impl Backend>, area: Rect) {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(0)
			.constraints([
				Constraint::Length(3),
				Constraint::Length(3),
				Constraint::Min(0),
			].as_ref())
			.split(area)
		;

		// 标签
		let tabs = Tabs::default()
			.block(Block::default().title("窗口标签").borders(Borders::ALL))
			.titles(&["Tab1", "Tab2", "Tab3", "Tab4", "Tab5", "Tab6"])
			.style(Style::default().fg(Color::White))
			.highlight_style(Style::default().fg(Color::Yellow))
			.divider("|")
		;
		f.render_widget(tabs, chunks[0]);

		// 路径
		let text = [Text::raw("~/")];
		let path = Paragraph::new(text.iter())
			.block(Block::default().title("路径").borders(Borders::ALL))
			.style(Style::default().fg(Color::White).bg(Color::Black))
			.wrap(true)
		;
		f.render_widget(path, chunks[1]);

		// 文件列表
		let row_style = Style::default().fg(Color::White);
		let files = Table::new(
				["Col1", "Col2", "Col3"].iter(),
				vec![
					Row::StyledData(["Row11", "Row12", "Row13"].iter(), row_style),
					Row::StyledData(["Row21", "Row22", "Row23"].iter(), row_style),
					Row::StyledData(["Row31", "Row32", "Row33"].iter(), row_style),
					Row::Data(["Row41", "Row42", "Row43"].iter())
				].into_iter()
			)
			.block(Block::default().title("文件列表").borders(Borders::ALL))
			.header_style(Style::default().fg(Color::Yellow))
			.widths(&[
				Constraint::Length(5),
				Constraint::Length(5),
				Constraint::Length(10)]
			)
			.style(Style::default().fg(Color::White))
			.column_spacing(1)
		;
		f.render_widget(files, chunks[2]);
	}
}
