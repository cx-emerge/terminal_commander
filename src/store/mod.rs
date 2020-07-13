mod file_window;

use file_window::FileWindow;


/// 状态库
pub struct Store {

	/// 当前激活的文件窗口
	pub active_window: usize,

	/// 窗口激活的标签
	active_tab: Vec<usize>,

	/// 文件窗口状态
	file_windows: Vec<Vec<FileWindow>>,

}


impl Store {

	/// 实例化 Store
	pub fn new() -> Self {
		return Store {
			active_window: 0,
			active_tab: vec![0, 0],
			file_windows: vec![
				vec![FileWindow::new(),],
				vec![FileWindow::new(),],
			],
		};
	}

	/// 切换窗口
	pub fn toggle_window(&mut self) {
		self.active_window = [1, 0][self.active_window];
	}

	/// 获取当前激活的文件窗口
	pub fn active_file_window(&mut self) -> &mut FileWindow {
		let window = self.active_window;
		let tab = self.active_tab[window];

		let file_window = self.file_window(window, tab);

		return file_window;
	}

	/// 获取文件窗口
	///
	/// - `window`: 窗口索引
	/// - `tab`: 标签索引
	pub fn file_window(&mut self,
		window: usize,
		tab: usize
	) -> &mut FileWindow {
		let ref mut file_window = self.file_windows[window][tab];

		return file_window;
	}

}
