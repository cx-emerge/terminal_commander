use dirs;


/// 文件窗口状态
pub struct FileWindow {
	/// 目录
	pub dir: String,

	/// 当前索引
	pub current_index: usize,
}


impl FileWindow {
	pub fn new() -> Self {
		let home_dir = dirs::home_dir().unwrap()
			.to_str().unwrap()
			.to_string()
		;

		return FileWindow {
			dir: home_dir,
			current_index: 0,
		};
	}

	/// 下一个文件
	pub fn next(&mut self) {
		self.current_index += 1;
	}

	/// 上一个文件
	pub fn prev(&mut self) {
		if self.current_index == 0 {
			return;
		}

		self.current_index -= 1;
	}

	/// 返回顶部
	pub fn back_to_top(&mut self) {
		self.current_index = 0;
	}
}
