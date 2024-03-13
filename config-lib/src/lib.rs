mod core;

#[cfg(test)]
mod tests {
    use crate::core::config_tree::ConfigTree;

    #[test]
    fn it_works() {
        let mut tree;
        match ConfigTree::new() {
            Ok(tree_val) => tree = tree_val,
            Err(msg) => panic!("{}", msg)
        };

        let val = tree.get("hello.key1.key2").or_else(|| Some("empty")).unwrap();
        let val2 = tree.get("hello.key1").or_else(|| Some("empty")).unwrap();
        println!("{val}");
        println!("{val2}");
    }
}
