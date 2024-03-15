use std::collections::HashMap;

use crate::core::config_reader;
use crate::utils::string_util;

const EMPTY_PARAMS: &'static str = "params is empty";

pub struct ConfigTree {
    root: Box<Node>,
}

impl ConfigTree {
    pub fn new() -> Result<ConfigTree, String> {
        let config: Vec<String> = match config_reader::read() {
            Ok(conf) => conf,
            Err(err) => return Err(err.to_string())
        };

        Self::new_from_config(config)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        let mut cur = &self.root;
        for part in key.split(".") {
            if !cur.children.contains_key(part) {
                return None;
            }
            cur = cur.children.get(part).expect("must contain key");
        }

        if let Some(val) = &cur.val {
            Some(&val)
        } else {
            None
        }
    }

    pub fn new_from_config(config: Vec<String>) -> Result<ConfigTree, String> {
        let root: Box<Node> = Node::new_empty();
        let mut tree = ConfigTree { root };

        for line in config {
            let kv: Vec<&str> = line.split("=").collect();
            if kv.len() != 2 {
                return Err(format!("incorrect config line format: {:?})", kv));
            }
            let key: &str = *kv.first().expect("key must exist");
            let val: &str = *kv.last().expect("val must exist");
            if string_util::has_space_char(key) {
                return Err(format!("key '{}' has whitespace char", key));
            }
            tree.add(String::from(key.trim()), String::from(val.trim()))?;
        }

        Ok(tree)
    }

    fn add(&mut self, key: String, val: String) -> Result<(), String> {
        if val.is_empty() || key.is_empty() {
            return Err(format!("params are empty: val {}, key {}", val, key));
        }
        let mut cur: &mut Box<Node> = &mut self.root;
        for part in key.split(".") {
            if !cur.children.contains_key(part) {
                cur.children.insert(String::from(part), Node::new_empty());
            }

            cur = cur.children.get_mut(part).expect("must contain key");
        }

        cur.val = Some(val);
        Ok(())
    }
}

struct Node {
    val: Option<String>,
    children: HashMap<String, Box<Node>>,
}

impl Node {
    fn new(val: String) -> Result<Box<Node>, &'static str> {
        let trim: &str = val.trim();
        if trim.is_empty() {
            return Err(EMPTY_PARAMS);
        }
        let trim_val: String = if trim.len() == val.len() { val } else { String::from(trim) };
        let node = Box::new(Node {
            val: Some(trim_val),
            children: HashMap::new(),
        });
        Ok(node)
    }

    fn new_empty() -> Box<Node> {
        Box::new(Node {
            val: None,
            children: HashMap::new(),
        })
    }
}
