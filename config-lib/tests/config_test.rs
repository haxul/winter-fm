#[cfg(test)]
mod tests {
    use config_lib::Config;

    #[test]
    fn get_regular_key() {
        let config = Config::new_from(vec!["hello.world=1".to_string(), "hello.world1=2".to_string()]).unwrap();

        assert_eq!("1", config.get("hello.world").unwrap());
        assert_eq!("2", config.get("hello.world1").unwrap())
    }

    #[test]
    fn get_overlapping_key() {
        let config = Config::new_from(vec!["hello.world=1".to_string(), "hello=2".to_string()]).unwrap();

        assert_eq!("1", config.get("hello.world").unwrap());
        assert_eq!("2", config.get("hello").unwrap())
    }

    #[test]
    fn get_not_existed_key() {
        let config = Config::new_from(vec!["hello.world=1".to_string(), "hello=2".to_string()]).unwrap();

        assert_eq!(true, config.get("world").is_none());
    }

    #[test]
    fn get_not_trimmed_key() {
        let config = Config::new_from(vec!["hello.world= 1 ".to_string(), "hello=2".to_string()]).unwrap();

        assert_eq!("1", config.get("hello.world").unwrap());
    }


    #[test]
    fn key_has_whitespaces() {
        let err = Config::new_from(vec!["hello.  world= 1 ".to_string()]).err().unwrap();

        assert_eq!("key 'hello.  world' has whitespace char", err)
    }
}
