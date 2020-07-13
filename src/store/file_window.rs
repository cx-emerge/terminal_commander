use std::{
	fs,
	path::Path,
};

use dirs;


/// 文件窗口状态
pub struct FileWindow {
	/// 目录
	pub dir: String,

	/// 当前索引
	pub current_index: usize,

	/// 列表
	pub list: Vec<fs::DirEntry>,
}


impl FileWindow {
	pub fn new() -> Self {
		let home_dir = dirs::home_dir().unwrap()
			.to_str().unwrap()
			.to_string()
		;

		let list = FileWindow::get_list(&home_dir);

		return FileWindow {
			dir: home_dir,
			current_index: 0,
			list: list,
		};
	}

	/// 下一个文件
	pub fn next(&mut self) {
		if self.current_index == self.list.len() - 1 {
			return;
		}

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

	/// 打开目录
	pub fn open(&mut self) {
		if self.list.len() == 0 {
			return;
		}

		let ref current = self.list[self.current_index];

		if !current.metadata().unwrap().is_dir() {
			return;
		}

		self.dir = String::from(
			current.path().to_str().unwrap()
		);

		self.update_list();
	}

	/// 返回上级目录
	pub fn up(&mut self) {
		let path = Path::new(&self.dir);

		let parent_dir = String::from(
			path.parent()
			.unwrap_or(Path::new("/"))
			.to_str().unwrap()
		);

		self.dir = parent_dir;

		self.update_list();
	}

	/// 更新列表
	pub fn update_list(&mut self) {
		let list = FileWindow::get_list(&self.dir);

		self.list = list;

		self.current_index = 0;
	}

	fn get_list(dir: &String) -> Vec<fs::DirEntry> {
		let mut list = fs::read_dir(dir).unwrap()
			.map(|entry| entry.unwrap())
			.collect::<Vec<_>>()
		;
		list.sort_by_key(|dir| dir.path().to_str().unwrap().to_lowercase());

		return list;
	}
}
