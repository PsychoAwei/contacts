use std::path::PathBuf;

/// 子命令/参数解析阶段的错误。
///
/// 用枚举而不是 String:错误种类是封闭的,调用方可以穷举 match,
/// 编译器也会帮忙检查有没有漏掉新加的分支。
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("缺少子命令\n用法: contacts <add|delete|find|list|help>")]
    MissingCommand,

    #[error("`{cmd}` 缺少参数 `{arg}`")]
    MissingArg { cmd: &'static str, arg: &'static str },

    #[error("`delete` 需要数字 id,但收到 {0:?}")]
    InvalidId(String),

    #[error("未知子命令 {0:?}\n用法: contacts <add|delete|find|list|help>")]
    UnknownCommand(String),
}

/// 读写数据文件时的错误。
///
/// IO 错误带上路径:只报 "No such file or directory" 的话,
/// 用户根本不知道是哪个文件出了问题。
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("无法读取 {0}: {1}")]
    Read(PathBuf, #[source] std::io::Error),

    #[error("无法写入 {0}: {1}")]
    Write(PathBuf, #[source] std::io::Error),

    #[error("JSON 处理失败: {0}")]
    Json(#[from] serde_json::Error),
}
