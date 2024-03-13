use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const CONFIG_FILE_PATH: &str = "resource/application.properties";

pub fn read() -> Result<Vec<String>, io::Error> {
    let file = File::open(CONFIG_FILE_PATH)?;
    let reader = BufReader::new(file);
    let mut vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        vec.push(line?);
    }
    Ok(vec)
}

