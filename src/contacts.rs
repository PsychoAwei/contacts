use crate::storage::Storage;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub id: u32,
    pub name: String,
    pub phone: String,
}

impl Contact {
    fn new(id: u32, name: String, phone: String) -> Self {
        Self { id, name, phone }
    }
}

pub struct Contacts {
    contacts: Vec<Contact>,
    next_id: u32,
}

impl Storage for Contacts {
    fn load(path: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        // 1.读取文件
        let content = fs::read_to_string(path)?;
        // 2. JSON -> Vec<Contact>
        let contacts: Vec<Contact> = serde_json::from_str(&content)?;
        // 3. 构造 Contacts
        Ok(Self {
            contacts,
            next_id: 1,
        })
    }
    fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        // Vec<Contact> -> JSON String
        let content = serde_json::to_string_pretty(&self.contacts)?;

        // JSON String -> 文件
        std::fs::write(path, content)?;

        Ok(())
    }
}

impl Contacts {
    pub fn add(&mut self, name: String, phone: String) {
        self.contacts.push(Contact::new(self.next_id, name, phone));
        self.next_id += 1;
    }
}
