use crate::command::{Command, USAGE};
use crate::contacts::{Contact, Contacts};
use crate::error::Error;
use std::path::PathBuf;

const DEFAULT_PATH: &str = "contacts.json";
const ENV_PATH: &str = "CONTACTS_PATH";

/// 数据文件位置:优先读环境变量 `CONTACTS_PATH`,否则用当前目录下的 contacts.json。
///
/// 硬编码 `./contacts.json` 意味着数据跟着"当前工作目录"跑 ——
/// 在 ~ 下和在 /tmp 下运行会读到两份不同的数据。有了环境变量,
/// 测试之间能互相隔离,将来想换成 XDG 数据目录也只改这一处。
pub fn contacts_path() -> PathBuf {
    std::env::var_os(ENV_PATH)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_PATH))
}

pub fn run(cmd: Command) -> Result<(), Error> {
    // help 只打印用法,不碰数据文件 —— 数据文件损坏时也应该能看帮助
    if matches!(cmd, Command::Help) {
        println!("{USAGE}");
        return Ok(());
    }

    let path = contacts_path();
    let mut contacts = Contacts::load(&path)?;
    let mut dirty = false;

    match cmd {
        Command::Add { name, phone } => {
            let id = contacts.add(name, phone);
            dirty = true;
            println!("已添加联系人,ID: {id}");
        }
        Command::Delete { id } => match contacts.delete(id) {
            Some(removed) => {
                dirty = true;
                println!("已删除联系人: {} (ID: {})", removed.name, removed.id);
            }
            None => println!("未找到 ID 为 {id} 的联系人"),
        },
        Command::Find { name } => {
            let found = contacts.find(&name);
            if found.is_empty() {
                println!("没有找到名字包含 {name:?} 的联系人");
            } else {
                for contact in found {
                    print_contact(contact);
                }
            }
        }
        Command::List => {
            let all = contacts.list();
            if all.is_empty() {
                println!("通讯录是空的,用 `contacts add <名字> <电话>` 添加一个");
            } else {
                for contact in all {
                    print_contact(contact);
                }
            }
        }
        // 函数开头已经提前返回
        Command::Help => unreachable!("help 在上面已处理"),
    }

    // 只在真的改了数据时才写盘
    if dirty {
        contacts.save(&path)?;
    }

    Ok(())
}

fn print_contact(contact: &Contact) {
    println!("ID: {}, Name: {}, Phone: {}", contact.id, contact.name, contact.phone);
}
