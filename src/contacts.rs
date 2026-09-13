use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub id: u32,
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Contacts {
    next_id: u32,
    contacts: Vec<Contact>,
}

impl Default for Contacts {
    fn default() -> Self {
        // id 从 1 开始。从 0 开始的话第一个联系人是 "ID: 0",对用户不直观。
        Self {
            next_id: 1,
            contacts: Vec::new(),
        }
    }
}

impl Contacts {
    pub fn new() -> Self {
        Self::default()
    }

    /// 从 JSON 文件加载。
    ///
    /// 文件不存在 = 还没有任何联系人,是正常状态而不是错误 ——
    /// 否则新用户第一次运行程序就会直接失败。
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => return Err(Error::Read(path.to_path_buf(), e)),
        };
        Ok(serde_json::from_str(&content)?)
    }

    /// 保存到 JSON 文件。
    ///
    /// 先写临时文件再 rename:直接覆盖原文件的话,写到一半被 Ctrl-C 或断电,
    /// 留下的就是半个 JSON,数据全没了。同目录的 rename 是原子操作,
    /// 读到的要么是完整的旧内容,要么是完整的新内容。
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path.as_ref();
        let content = serde_json::to_string_pretty(self)?;

        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, content).map_err(|e| Error::Write(tmp.clone(), e))?;
        fs::rename(&tmp, path).map_err(|e| Error::Write(path.to_path_buf(), e))?;
        Ok(())
    }

    /// 添加联系人,返回分配到的 id。
    pub fn add(&mut self, name: String, phone: String) -> u32 {
        let id = self.next_id;
        self.contacts.push(Contact { id, name, phone });
        self.next_id += 1;
        id
    }

    /// 按 id 删除,返回被删掉的那个联系人。
    ///
    /// 返回 `Option<Contact>` 而不是 `bool`:调用方拿到名字,
    /// 才能打印 "已删除 luowei" 而不是干巴巴的 "已删除"。
    pub fn delete(&mut self, id: u32) -> Option<Contact> {
        let pos = self.contacts.iter().position(|c| c.id == id)?;
        Some(self.contacts.remove(pos))
    }

    pub fn list(&self) -> &[Contact] {
        &self.contacts
    }

    /// 按名字模糊查找,忽略大小写。
    pub fn find(&self, name: &str) -> Vec<&Contact> {
        let needle = name.to_lowercase();
        self.contacts
            .iter()
            .filter(|c| c.name.to_lowercase().contains(&needle))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Contacts {
        let mut c = Contacts::new();
        c.add("luowei".into(), "18537721985".into());
        c.add("tong".into(), "1234567855".into());
        c
    }

    #[test]
    fn ids_start_at_one_and_increment() {
        let c = sample();
        let ids: Vec<u32> = c.list().iter().map(|x| x.id).collect();
        assert_eq!(ids, vec![1, 2]);
    }

    #[test]
    fn delete_returns_the_removed_contact() {
        let mut c = sample();
        let removed = c.delete(1).expect("id 1 应该存在");
        assert_eq!(removed.name, "luowei");
        assert_eq!(c.list().len(), 1);
    }

    #[test]
    fn delete_unknown_id_changes_nothing() {
        let mut c = sample();
        assert!(c.delete(999).is_none());
        assert_eq!(c.list().len(), 2);
    }

    #[test]
    fn find_ignores_case_and_matches_substrings() {
        let c = sample();
        assert_eq!(c.find("LUO").len(), 1);
        assert_eq!(c.find("o").len(), 2); // luowei 和 tong 都含 o
        assert!(c.find("nobody").is_empty());
    }

    #[test]
    fn loading_a_missing_file_gives_an_empty_address_book() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("does-not-exist.json");

        let mut c = Contacts::load(&path).unwrap();
        assert!(c.list().is_empty());
        assert_eq!(c.add("a".into(), "1".into()), 1);
    }

    #[test]
    fn save_then_load_roundtrips_and_keeps_counter() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("contacts.json");
        sample().save(&path).unwrap();

        let mut reloaded = Contacts::load(&path).unwrap();
        assert_eq!(reloaded.list(), sample().list());
        // 关键:重新加载后接着发号,而不是从 1 重来导致 id 冲突
        assert_eq!(reloaded.add("new".into(), "0".into()), 3);
    }

    #[test]
    fn save_leaves_no_temp_file_behind() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("contacts.json");
        sample().save(&path).unwrap();

        let files: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        assert_eq!(files.len(), 1, "临时文件应该已经被 rename 掉:{files:?}");
    }

    #[test]
    fn empty_json_object_falls_back_to_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("contacts.json");
        fs::write(&path, "{}").unwrap();

        let mut c = Contacts::load(&path).unwrap();
        assert_eq!(c.add("a".into(), "1".into()), 1);
    }
}
