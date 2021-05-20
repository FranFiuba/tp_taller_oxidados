use core::fmt::{Display, Formatter, Result};
use std::collections::HashMap;

pub struct Database {
    dictonary: HashMap<String, Type>,
}

pub enum Type {
    String(String),
    List(Vec<String>),
}

impl Database {
    pub fn new() -> Database {
        Database {
            dictonary: HashMap::new(),
        }
    }

    pub fn append(&mut self, key: &str, value: &str) {
        if self.dictonary.contains_key(key) {
            if let Some(Type::String(val)) = self.dictonary.get_mut(key) {
                val.push_str(value);
            }
        } else {
            self.dictonary
                .insert(String::from(key), Type::String(String::from(value)));
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Type::String(val) => write!(f, "{:?}", val),
            Type::List(val) => {
                for element in val {
                    write!(f, "{}", element);
                }

                Ok(())
            }
        }
    }
}



impl Display for Database {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (key, value) in self.dictonary.iter() {
            write!(f, "key: {}, value: {}\n", key, value);
        }

        Ok(())
    }
}
