#![allow(dead_code)]

use std::collections::HashSet;

pub fn intern_work(input: Vec<String>) -> i32 {
    let wowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let disallowed = HashSet::from(["ab", "cd", "pq", "xy"]);

    let mut count = 0;

    let mut wowel_num = 0;
    let mut not_allowed = false;
    let mut twice_appeared = false;

    for string in input {
        let b = string.as_bytes();
        let ln = string.chars().count() - 1;

        for (i, ch) in b.iter().enumerate() {
            let first = *ch as char;

            if i < ln {
                let second = b[i + 1] as char;

                let merged = format!("{}{}", first, second);

                if disallowed.contains(merged.as_str()) {
                    not_allowed = true;
                    break;
                }

                if first == second {
                    twice_appeared = true;
                }
            }

            if wowels.contains(&first) {
                wowel_num += 1;
            }
        }

        if !not_allowed && wowel_num >= 3 && twice_appeared {
            count += 1;
        }

        wowel_num = 0;
        not_allowed = false;
        twice_appeared = false;
    }

    count
}
