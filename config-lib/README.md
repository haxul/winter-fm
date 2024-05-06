Environment config library for Rust <p>

Default file with configs must be placed in <i>recourse/application.properties</i> file<p>

Application.properties has format ```key=value```. <p>
Example:

```
k1.k2.k3=v3
k1=v1
k1.k2=v2
```

Lib usage:

1. Create Config object
2. Use method ```get(key:String)``` of Config Object

Example: <p>

```
fn main() {
    let config = Config::new();
    let value = config.get("k1").unwrap();
    
    assert_eq!("v1", value);
}
```


