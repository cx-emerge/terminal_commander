mod file_window;

use file_window::FileWindow;


/// 状态库
pub struct Store {

	/// 当前激活的文件窗口
	active_file_window: usize,

	/// 文件窗口状态
	file_windows: Vec<FileWindow>,

}


impl Store {

	/// 实例化 Store
	pub fn new() -> Self {
		return Store {
			active_file_window: 0,
			file_windows: vec![
				FileWindow::new(),
				FileWindow::new(),
			],
		};
	}

	/// 切换窗口
	pub fn toggle_window(&mut self) {
		self.active_file_window = [1, 0][self.active_file_window];
	}

	/// 获取当前激活的文件窗口
	pub fn active_file_window(&mut self) -> &mut FileWindow {
		let file_window = self.file_window(self.active_file_window);

		return file_window;
	}

	/// 获取文件窗口
	pub fn file_window(&mut self, index: usize) -> &mut FileWindow {
		let ref mut file_window = self.file_windows[index];

		return file_window;
	}

}
