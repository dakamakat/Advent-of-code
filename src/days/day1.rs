#![allow(dead_code)]

use std::collections::HashMap;

use crate::utils;

pub fn floor(input: String) -> i32 {
    let mut map = HashMap::from([('(', 0), (')', 0)]);

    for sm in input.chars() {
        *map.get_mut(&sm).unwrap() += 1;
    }

    map[&'('] - map[&')']
}

pub fn first_cause(input: String, cause: i32) -> i32 {
    let mut map = HashMap::from([('(', 0), (')', 0)]);

    for (i, sm) in input.chars().enumerate() {
        *map.get_mut(&sm).unwrap() += 1;

        if cause == map[&'('] - map[&')'] {
            return (i + 1) as i32;
        }
    }

    0
}

pub fn exec() {
    let input = utils::read_signle("./inputs/day1.txt".to_string());

    let result = floor(input.clone());
    print!("{0}", result);

    let cause = first_cause(input.to_string(), -1);
    print!("{0}", cause);
}
