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

	/// 列表过滤词
	pub filter: String,
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
			filter: String::new(),
		};
	}

	/// 下一个文件
	pub fn next(&mut self) {
		if self.list.is_empty() || self.current_index >= self.list.len() - 1 {
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

	/// 更新列表过滤词
	pub fn set_filter(&mut self, filter: &str) {
		self.filter = filter.to_string();
		self.update_list();
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

	/// 在当前目录创建目录
	pub fn create_directory(&mut self, name: &str) -> std::io::Result<()> {
		fs::create_dir(Path::new(&self.dir).join(name))?;
		self.update_list_at(name);
		Ok(())
	}

	/// 在当前目录创建文件
	pub fn create_file(&mut self, name: &str) -> std::io::Result<()> {
		fs::OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(Path::new(&self.dir).join(name))?;
		self.update_list_at(name);
		Ok(())
	}

	/// 重命名当前选中项
	pub fn rename_current(&mut self, name: &str) -> std::io::Result<()> {
		let Some(current) = self.list.get(self.current_index) else {
			return Ok(());
		};
		fs::rename(current.path(), Path::new(&self.dir).join(name))?;
		self.update_list_at(name);
		Ok(())
	}

	/// 将当前选中项移至回收站
	pub fn delete_current(&mut self) -> Result<(), trash::Error> {
		let Some(current) = self.list.get(self.current_index) else {
			return Ok(());
		};
		trash::delete(current.path())?;
		let previous_index = self.current_index;
		self.list = FileWindow::get_list(&self.dir);
		self.current_index = previous_index.min(self.list.len().saturating_sub(1));
		Ok(())
	}

	/// 将当前选中项复制到另一栏
	pub fn copy_current_to(&self, destination: &mut FileWindow, overwrite: bool) -> std::io::Result<()> {
		let Some(current) = self.list.get(self.current_index) else {
			return Ok(());
		};

		let source = current.path();
		let name = current.file_name();
		let target = Path::new(&destination.dir).join(&name);
		if source.is_dir() && target.starts_with(&source) {
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidInput,
				"cannot copy a directory into itself",
			));
		}
		if fs::symlink_metadata(&target).is_ok() {
			if !overwrite {
				return Err(std::io::Error::new(
					std::io::ErrorKind::AlreadyExists,
					"destination already exists",
				));
			}
			if fs::symlink_metadata(&target)?.file_type().is_dir() {
				fs::remove_dir_all(&target)?;
			} else {
				fs::remove_file(&target)?;
			}
		}

		FileWindow::copy_path(&source, &target)?;
		destination.update_list_at(&name.to_string_lossy());
		Ok(())
	}

	/// 更新列表
	pub fn update_list(&mut self) {
		let filter = self.filter.to_lowercase();
		let list = FileWindow::get_list(&self.dir)
			.into_iter()
			.filter(|entry| entry.file_name().to_string_lossy().to_lowercase().contains(&filter))
			.collect();

		self.list = list;

		self.current_index = 0;
	}

	fn update_list_at(&mut self, name: &str) {
		self.update_list();
		self.current_index = self.list.iter()
			.position(|entry| entry.file_name() == name)
			.unwrap_or(0);
	}

	fn get_list(dir: &String) -> Vec<fs::DirEntry> {
		let Ok(entries) = fs::read_dir(dir) else {
			return Vec::new();
		};
		let mut list = entries.filter_map(Result::ok).collect::<Vec<_>>();
		list.sort_by_key(|entry| entry.path().to_string_lossy().to_lowercase());

		return list;
	}

	fn copy_path(source: &Path, destination: &Path) -> std::io::Result<()> {
		if source.is_dir() {
			fs::create_dir_all(destination)?;
			for entry in fs::read_dir(source)? {
				let entry = entry?;
				FileWindow::copy_path(&entry.path(), &destination.join(entry.file_name()))?;
			}
		} else {
			fs::copy(source, destination)?;
		}
		Ok(())
	}
}
