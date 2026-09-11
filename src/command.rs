#[derive(Debug)]
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
