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
		let tabs_widget = Tabs::default()
			.block(Block::default()
				.title("窗口标签")
				.title_style(Style::default().fg(Color::Gray))
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Gray))
			)
			.titles(&["projects"])
			.style(Style::default().fg(Color::White))
			.highlight_style(Style::default().fg(Color::Yellow))
			.divider("|")
		;
		f.render_widget(tabs_widget, chunks[0]);

		// 路径
		let text = [Text::raw("~/")];
		let path_widget = Paragraph::new(text.iter())
			.block(Block::default()
				.title("路径")
				.title_style(Style::default().fg(Color::Gray))
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Gray))
			)
			.style(Style::default().fg(Color::White).bg(Color::Black))
			.wrap(true)
		;
		f.render_widget(path_widget, chunks[1]);

		// 文件列表
		let table_widths = [
			Constraint::Length(area.width - 15 - 15),
			Constraint::Length(15),
			Constraint::Length(15),
		];
		let files_widget = Table::new(
				["文件名", "文件大小", "修改时间",].iter(),
				vec![
					Row::Data(["file_01.md", "100kb", "2020-7-7"].iter()),
					Row::Data(["file_02.md", "100kb", "2020-7-7"].iter()),
					Row::Data(["file_03.md", "100kb", "2020-7-7"].iter()),
				].into_iter()
			)
			.block(Block::default().title("文件列表").borders(Borders::ALL))
			.header_style(Style::default().fg(Color::Yellow))
			.widths(&table_widths)
			.style(Style::default().fg(Color::White))
			.highlight_style(Style::default().fg(Color::Yellow))
			.column_spacing(1)
		;
		f.render_widget(files_widget, chunks[2]);
	}
}
