#[derive(Debug, PartialEq)]
pub enum Command {
    Add(String, String),
    Delete(u32),
    Find(String),
    List,
}

impl Command {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        // 把程序名去掉
        args.next();
        // 得到命令
        let cmd: String = args.next().ok_or("缺少子命令".to_string())?;
        match cmd.as_str() {
            "add" => {
                let name = args.next().ok_or("add命令缺少名字和电话号码".to_string())?;
                let phone = args.next().ok_or("add命令缺少电话号码".to_string())?;
                Ok(Command::Add(name, phone))
            }
            "delete" => {
                let id = args
                    .next()
                    .ok_or("delete命令缺少要删除对象的id".to_string())?;
                let id: u32 = id.parse().map_err(|_| "仅支持用id删除联系人".to_string())?;
                Ok(Command::Delete(id))
            }
            "find" => {
                let name = args
                    .next()
                    .ok_or("find命令缺少查找对象的名字".to_string())?;
                Ok(Command::Find(name))
            }
            "list" => Ok(Command::List),
            _ => Err("仅支持add,delete,find,list四个命令".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// parse 期望迭代器的第一项是程序名，这里统一补上，方便测试
    fn parse_args(args: &[&str]) -> Result<Command, String> {
        let argv = std::iter::once("contacts".to_string())
            .chain(args.iter().map(|arg| arg.to_string()));
        Command::parse(argv)
    }

    #[test]
    fn parses_add() {
        assert_eq!(
            parse_args(&["add", "alice", "13800138000"]),
            Ok(Command::Add("alice".to_string(), "13800138000".to_string()))
        );
    }

    #[test]
    fn parses_delete() {
        assert_eq!(parse_args(&["delete", "42"]), Ok(Command::Delete(42)));
    }

    #[test]
    fn parses_delete_id_boundaries() {
        assert_eq!(parse_args(&["delete", "0"]), Ok(Command::Delete(0)));
        assert_eq!(
            parse_args(&["delete", "4294967295"]),
            Ok(Command::Delete(u32::MAX))
        );
    }

    #[test]
    fn parses_find() {
        assert_eq!(
            parse_args(&["find", "bob"]),
            Ok(Command::Find("bob".to_string()))
        );
    }

    #[test]
    fn parses_list() {
        assert_eq!(parse_args(&["list"]), Ok(Command::List));
    }

    #[test]
    fn reports_missing_arguments() {
        let cases: &[(&[&str], &str)] = &[
            (&[], "缺少子命令"),
            (&["add"], "add命令缺少名字和电话号码"),
            (&["add", "alice"], "add命令缺少电话号码"),
            (&["delete"], "delete命令缺少要删除对象的id"),
            (&["find"], "find命令缺少查找对象的名字"),
        ];

        for &(args, expected) in cases {
            match parse_args(args) {
                Ok(cmd) => panic!("parse_args({args:?}) 应该报错，却返回了 {cmd:?}"),
                Err(err) => assert_eq!(err, expected, "parse_args({args:?}) 的错误信息不对"),
            }
        }
    }

    #[test]
    fn rejects_invalid_id() {
        let cases: &[&[&str]] = &[
            &["delete", "abc"],        // 不是数字
            &["delete", "-1"],         // 负数
            &["delete", "1.5"],        // 小数
            &["delete", "4294967296"], // 超出 u32 范围
            &["delete", ""],           // 空字符串
        ];

        for &args in cases {
            match parse_args(args) {
                Ok(cmd) => panic!("parse_args({args:?}) 应该报错，却返回了 {cmd:?}"),
                Err(err) => assert_eq!(err, "仅支持用id删除联系人", "parse_args({args:?})"),
            }
        }
    }

    #[test]
    fn rejects_unknown_command() {
        assert_eq!(
            parse_args(&["remove", "1"]).unwrap_err(),
            "仅支持add,delete,find,list四个命令"
        );
    }

    #[test]
    fn ignores_extra_arguments() {
        // 记录当前行为：多余参数会被直接忽略，不会报错
        assert_eq!(parse_args(&["list", "多余的参数"]), Ok(Command::List));
        assert_eq!(
            parse_args(&["find", "bob", "多余"]),
            Ok(Command::Find("bob".to_string()))
        );
    }
}
