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

#[derive(Debug, Serialize, Deserialize)]
pub struct Contacts {
    next_id: u32,
    contacts: Vec<Contact>,
}

impl Storage for Contacts {
    fn load(path: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        // 1.读取文件
        let content = fs::read_to_string(path)?;
        // 2. JSON -> Vec<Contact>
        let contacts: Self = serde_json::from_str(&content)?;
        // 3. 构造 Contacts
        Ok(contacts)
    }
    fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        // Vec<Contact> -> JSON String
        let content = serde_json::to_string_pretty(self)?;

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

    pub fn delete(&mut self, id: u32) -> bool {
        let old_len = self.contacts.len();
        self.contacts.retain(|contact| contact.id != id);
        let is_deleted = self.contacts.len() != old_len;
        is_deleted
    }

    pub fn list(&self) -> &[Contact] {
        &self.contacts
    }

    pub fn find(&self, name: &str) -> Vec<&Contact> {
        self.contacts
            .iter()
            .filter(|contact| contact.name.contains(name))
            .collect()
    }
}
