//! 按键配置


use crossterm::event::{
	KeyEvent,
	KeyCode,
	KeyModifiers,
};


/// 退出
pub const QUIT: KeyEvent = KeyEvent {
	code: KeyCode::Char('c'),
	modifiers: KeyModifiers::CONTROL,
};


/// 窗口切换
pub const TOGGLE_WINDOW: KeyEvent = KeyEvent {
	code: KeyCode::Tab,
	modifiers: KeyModifiers::NONE,
};


/// 文件列表光标下移
pub const FILES_CURSOR_NEXT: KeyEvent = KeyEvent {
	code: KeyCode::Char('j'),
	modifiers: KeyModifiers::NONE,
};


/// 文件列表光标上移
pub const FILES_CURSOR_PREV: KeyEvent = KeyEvent {
	code: KeyCode::Char('k'),
	modifiers: KeyModifiers::NONE,
};


/// 文件列表光标移至顶部
pub const FILES_CURSOR_TOP: KeyEvent = KeyEvent {
	code: KeyCode::Char('g'),
	modifiers: KeyModifiers::NONE,
};


/// 打开目录
pub const OPEN_DIR: KeyEvent = KeyEvent {
	code: KeyCode::Char('o'),
	modifiers: KeyModifiers::NONE,
};


/// 返回上级目录
pub const BACK_TO_THE_PARENT_DIRECTORY: KeyEvent = KeyEvent {
	code: KeyCode::Char('u'),
	modifiers: KeyModifiers::NONE,
};
