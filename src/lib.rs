//! 一个用 JSON 文件存数据的命令行通讯录。
//!
//! 分成两层:
//! - [`command`] 和 [`cli`] —— 命令行这一侧,参数解析和输出
//! - [`contacts`] —— 数据模型和增删查改,不关心自己是怎么被调用的
//!
//! 把 `cli` 放在库里而不是 bin 里,是为了让集成测试够得着它。

pub mod cli;
pub mod command;
pub mod contacts;
pub mod error;
