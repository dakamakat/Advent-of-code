use std::fs::{self, read_to_string};

pub fn read_signle(path: String) -> String {
    let str = fs::read_to_string(path).unwrap();
    str
}

pub fn read_lines(path: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
