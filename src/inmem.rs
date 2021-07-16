use crate::Command;

use std::collections::HashMap;

pub struct InMem {
    map: HashMap<String, String>,
}

impl InMem {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn execute(&mut self, command: &str) -> Result<String, String> {
        match command.parse()? {
            Command::Get(key) => match self.map.get(&key) {
                Some(value) => Ok(value.to_owned()),
                None => Err(String::from("None")),
            },
            Command::Set(key, value) => match self.map.insert(key.to_owned(), value.to_owned()) {
                Some(old_value) => Ok(format!(
                    "Updated key '{}' from '{}' to '{}'",
                    key, old_value, value
                )),
                None => Ok(format!("Key '{}' set to '{}'", key, value)),
            },
            Command::Del(key) => match self.map.remove(&key) {
                Some(value) => Ok(format!("Previous value: '{}'", value)),
                None => Err(String::from("None")),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::InMem;
    #[test]
    fn inmem_test() {
        let mut inmem = InMem::new();

        assert_eq!(
            inmem.execute("set lang rust"),
            Ok(String::from("Key 'lang' set to 'rust'"))
        );

        assert_eq!(inmem.execute("get lang"), Ok(String::from("rust")));

        assert_eq!(
            inmem.execute("set lang c"),
            Ok(String::from("Updated key 'lang' from 'rust' to 'c'"))
        );

        assert_eq!(
            inmem.execute("del lang"),
            Ok(String::from("Previous value: 'c'"))
        );

        assert_eq!(inmem.execute("get lang"), Err(String::from("None")));

        assert_eq!(
            inmem.execute("set \"programming language\" \"The Rust Programming Language\""),
            Ok(String::from(
                "Key 'programming language' set to 'The Rust Programming Language'"
            ))
        );
    }
}
