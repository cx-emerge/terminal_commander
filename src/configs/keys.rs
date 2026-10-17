//! 按键配置


use crossterm::event::{KeyCode, KeyModifiers};


/// 退出
pub const QUIT: (KeyCode, KeyModifiers) = (KeyCode::Char('c'), KeyModifiers::CONTROL);


/// 窗口切换
pub const TOGGLE_WINDOW: (KeyCode, KeyModifiers) = (KeyCode::Tab, KeyModifiers::NONE);


/// 文件列表光标下移
pub const FILES_CURSOR_NEXT: (KeyCode, KeyModifiers) = (KeyCode::Char('j'), KeyModifiers::NONE);


/// 文件列表光标上移
pub const FILES_CURSOR_PREV: (KeyCode, KeyModifiers) = (KeyCode::Char('k'), KeyModifiers::NONE);


/// 文件列表光标移至顶部
pub const FILES_CURSOR_TOP: (KeyCode, KeyModifiers) = (KeyCode::Char('g'), KeyModifiers::NONE);


/// 打开目录
pub const OPEN_DIR: (KeyCode, KeyModifiers) = (KeyCode::Char('o'), KeyModifiers::NONE);


/// 返回上级目录
pub const BACK_TO_THE_PARENT_DIRECTORY: (KeyCode, KeyModifiers) = (KeyCode::Char('u'), KeyModifiers::NONE);


/// 新建目录
pub const CREATE_DIRECTORY: (KeyCode, KeyModifiers) = (KeyCode::F(7), KeyModifiers::NONE);


/// 重命名
pub const RENAME: (KeyCode, KeyModifiers) = (KeyCode::F(2), KeyModifiers::NONE);
