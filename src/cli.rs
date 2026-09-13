use contacts::command::Command;
use contacts::contacts::Contacts;
use contacts::storage::Storage;
const CONTACTS_PATH: &str = "./contacts.json";

pub fn run(cmd: Command) -> Result<(), Box<dyn std::error::Error>> {
    let mut contacts = Contacts::load(CONTACTS_PATH)?;
    match cmd {
        Command::Add(name, phone) => {
            contacts.add(name, phone);
            println!("联系人已添加");
            contacts.save(CONTACTS_PATH)?;
        }
        Command::Delete(id) => {
            if contacts.delete(id) {
                println!("联系人已删除");
                contacts.save(CONTACTS_PATH)?;
            } else {
                println!("未找到联系人");
            }
        }
        Command::Find(name) => {
            for contact in contacts.find(&name) {
                println!(
                    "ID: {}, Name: {}, Phone: {}",
                    contact.id, contact.name, contact.phone
                );
            }
        }
        Command::List => {
            for contact in contacts.list() {
                println!(
                    "ID: {}, Name: {}, Phone: {}",
                    contact.id, contact.name, contact.phone
                );
            }
        }
    }
    Ok(())
}
