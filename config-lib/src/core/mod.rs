use std::{io};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Map;

const CONFIG_FILE_PATH: &str = "resource/application.properties";

fn read_config_file() -> Result<Vec<String>, io::Error> {
    let file = File::open(CONFIG_FILE_PATH)?;
    let reader = BufReader::new(file);
    let mut vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        vec.push(line?);
    }
    Ok(vec)
}


struct ConfigTree {
    root: Box<Node>,
}

impl ConfigTree {
    pub fn new() -> Option<ConfigTree> {
        let root = Node::new(String::from("$"))?;
        Some(ConfigTree {
            root: Box::new(root)
        })
    }

    pub fn add(&self, key: String, val: String) -> Result<(), String> {
        if val.is_empty() {
            Err("val is empty")
        }

        if key.is_empty() {
            Err("key is empty")
        }

        let parts = key.split(".").collect::<Vec<&str>>();

        self.add_parts(parts, val)
    }

    fn add_parts(&self, key_parts: Vec<&str>, val: String) -> Result<(), String> {
        if key_parts.is_empty() {
            Ok(())
        }

        let mut cur = &self.root;
        for part in key_parts {
            //TODO
            cur.children.get(part)

        }

        Ok(())
    }
}

struct Node {
    val: String,
    children: HashMap<String, Box<Node>>,
}

impl Node {
    pub fn new(val: String) -> Option<Node> {
        let trim: &str = val.trim();
        if trim.is_empty() {
            None
        } else {
            let s: String = if trim.len() == val.len() { val } else { trim.to_string() };
            Some(Node {
                val: s,
                children: HashMap::new(),
            })
        }
    }
}

