//! 全局状态

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

	/// 将当前选中项复制到另一栏
	pub fn copy_current_to_other_window(&mut self, overwrite: bool) -> std::io::Result<()> {
		let source_window = self.active_window;
		let destination_window = 1 - source_window;
		let source_tab = self.active_tab[source_window];
		let destination_tab = self.active_tab[destination_window];
		let (source, destination) = if source_window == 0 {
			let (left, right) = self.file_windows.split_at_mut(1);
			(&left[0][source_tab], &mut right[0][destination_tab])
		} else {
			let (left, right) = self.file_windows.split_at_mut(1);
			(&right[0][source_tab], &mut left[0][destination_tab])
		};

		source.copy_current_to(destination, overwrite)
	}

	/// 删除当前选中项，并刷新指向相同目录的另一栏
	pub fn delete_current(&mut self) -> Result<(), trash::Error> {
		let source_window = self.active_window;
		let destination_window = 1 - source_window;
		let source_tab = self.active_tab[source_window];
		let destination_tab = self.active_tab[destination_window];
		let (source_dir, result) = {
			let source = self.file_window(source_window, source_tab);
			let dir = source.dir.clone();
			let result = source.delete_current();
			(dir, result)
		};
		let destination = self.file_window(destination_window, destination_tab);
		if destination.dir == source_dir {
			destination.update_list();
		}
		result
	}

}
