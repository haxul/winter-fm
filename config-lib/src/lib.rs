use crate::core::config_tree::ConfigTree;

mod core;
mod utils;


pub struct Config {
    tree: ConfigTree,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let tree = ConfigTree::new()?;
        Ok(Config { tree })
    }

    pub fn new_from(vec: Vec<String>) -> Result<Config, String> {
        let tree = ConfigTree::new_from_config(vec)?;
        Ok(Config { tree })
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.tree.get(key)
    }
}
