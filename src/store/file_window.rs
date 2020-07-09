/// 文件窗口状态
pub struct FileWindow {
	/// 当前文件索引
	pub file_index: usize,
}


impl FileWindow {
	pub fn new() -> Self {
		return FileWindow {
			file_index: 0,
		};
	}

	/// 下一个文件
	pub fn next(&mut self) {
		self.file_index += 1;
	}

	/// 上一个文件
	pub fn prev(&mut self) {
		if self.file_index == 0 {
			return;
		}

		self.file_index -= 1;
	}

	/// 返回顶部
	pub fn back_to_top(&mut self) {
		self.file_index = 0;
	}
}
