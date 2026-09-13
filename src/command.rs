use crate::error::ParseError;

pub const USAGE: &str = "\
用法: contacts <命令> [参数...]

命令:
  add <名字> <电话>    添加联系人
  delete <id>          按 id 删除联系人
  find <名字>          按名字查找(模糊匹配,忽略大小写)
  list                 列出所有联系人
  help                 显示这份帮助";

#[derive(Debug, PartialEq)]
pub enum Command {
    // 带字段名的结构体变体:调用处 `Command::Add { name, phone }` 一眼能看出
    // 两个 String 各是什么,元组变体 `Add(String, String)` 就得靠猜了。
    Add { name: String, phone: String },
    Delete { id: u32 },
    Find { name: String },
    List,
    Help,
}

impl Command {
    /// 解析参数。
    ///
    /// 这里只接收参数本身,程序名由调用方用 `env::args().skip(1)` 去掉 ——
    /// 把 "第一个元素是程序名" 这个约定藏在函数里,调用方很容易踩坑。
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, ParseError> {
        let cmd = args.next().ok_or(ParseError::MissingCommand)?;

        match cmd.as_str() {
            "add" => {
                let name = args
                    .next()
                    .ok_or(ParseError::MissingArg { cmd: "add", arg: "名字" })?;
                let phone = args
                    .next()
                    .ok_or(ParseError::MissingArg { cmd: "add", arg: "电话" })?;
                Ok(Command::Add { name, phone })
            }
            "delete" => {
                let raw = args
                    .next()
                    .ok_or(ParseError::MissingArg { cmd: "delete", arg: "id" })?;
                let id = raw.parse().map_err(|_| ParseError::InvalidId(raw))?;
                Ok(Command::Delete { id })
            }
            "find" => {
                let name = args
                    .next()
                    .ok_or(ParseError::MissingArg { cmd: "find", arg: "名字" })?;
                Ok(Command::Find { name })
            }
            "list" => Ok(Command::List),
            "help" | "-h" | "--help" => Ok(Command::Help),
            other => Err(ParseError::UnknownCommand(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Command, ParseError> {
        Command::parse(args.iter().map(|s| s.to_string()))
    }

    #[test]
    fn parses_add() {
        assert_eq!(
            parse(&["add", "luowei", "123"]).unwrap(),
            Command::Add { name: "luowei".into(), phone: "123".into() }
        );
    }

    #[test]
    fn parses_delete() {
        assert_eq!(parse(&["delete", "7"]).unwrap(), Command::Delete { id: 7 });
    }

    #[test]
    fn parses_help_aliases() {
        for arg in ["help", "-h", "--help"] {
            assert_eq!(parse(&[arg]).unwrap(), Command::Help);
        }
    }

    #[test]
    fn rejects_non_numeric_id() {
        assert!(matches!(
            parse(&["delete", "abc"]),
            Err(ParseError::InvalidId(_))
        ));
    }

    #[test]
    fn rejects_missing_arguments() {
        assert!(matches!(parse(&[]), Err(ParseError::MissingCommand)));
        assert!(matches!(
            parse(&["add", "only-a-name"]),
            Err(ParseError::MissingArg { .. })
        ));
        assert!(matches!(
            parse(&["find"]),
            Err(ParseError::MissingArg { .. })
        ));
    }

    #[test]
    fn rejects_unknown_command() {
        assert!(matches!(
            parse(&["frobnicate"]),
            Err(ParseError::UnknownCommand(_))
        ));
    }
}
