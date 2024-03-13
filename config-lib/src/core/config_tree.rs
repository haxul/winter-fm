use std::collections::HashMap;

use crate::core::config_reader;

const EMPTY_PARAMS: &str = "params is empty";

pub struct ConfigTree {
    root: Box<Node>,
}

impl ConfigTree {
    pub fn new() -> Result<ConfigTree, String> {
        let config = match config_reader::read() {
            Ok(conf) => conf,
            Err(err) => return Err(err.to_string())
        };

        Self::new_config_vec(config)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        let mut cur = &self.root;
        for part in key.split(".") {
            if !cur.children.contains_key(part) {
                return None;
            }
            cur = cur.children.get(part).expect("must contain key");
        }

        Some(&cur.val)
    }

    pub fn new_config_vec(config: Vec<String>) -> Result<ConfigTree, String> {
        let root = Node::new(String::from("$"))?;
        let mut tree = ConfigTree {
            root: Box::new(root)
        };

        for line in config {
            let kv: Vec<&str> = line.split("=").collect();
            if kv.len() != 2 {
                return Err(format!("incorrect config line format: {:?})", kv));
            }
            let key: &str = *kv.first().expect("key must exist");
            let val: &str = *kv.last().expect("val must exists");
            tree.add(key.to_string(), val.to_string())?;
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
                cur.children.insert(part.to_string(), Box::new(Node::new(part.to_string()).unwrap()));
            }

            cur = cur.children.get_mut(part).unwrap();
        }

        cur.val = val;
        Ok(())
    }
}

struct Node {
    val: String,
    children: HashMap<String, Box<Node>>,
}

impl Node {
    fn new(val: String) -> Result<Node, String> {
        let trim: &str = val.trim();
        if trim.is_empty() {
            return Err(EMPTY_PARAMS.to_string());
        }
        let trim_val: String = if trim.len() == val.len() { val } else { trim.to_string() };
        Ok(Node {
            val: trim_val,
            children: HashMap::new(),
        })
    }
}
