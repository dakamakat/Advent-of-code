use std::collections::HashMap;

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
