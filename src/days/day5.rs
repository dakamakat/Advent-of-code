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

pub fn intern_work_second_iteration(input: Vec<String>) -> i32 {
    let mut count = 0;

    let mut repeats = false;
    let mut twice_appeared = false;

    for string in input {
        let b = string.as_bytes();
        let ln = string.chars().count() - 1;

        for (i, ch) in b.iter().enumerate() {
            let first = *ch as char;

            let mut next = i + 1;
            if !repeats && next <= ln {
                let second = b[next] as char;
                let merged = format!("{}{}", first, second);

                let mut cloned_str = string.clone();
                cloned_str.replace_range(i..next + 1, "0");

                if cloned_str.contains(&merged) {
                    repeats = true;
                }
            }

            next = i + 2;
            if !twice_appeared && next <= ln {
                let second = b[next] as char;

                if first == second {
                    twice_appeared = true;
                }
            }

            if repeats && twice_appeared {
                break;
            }
        }

        if repeats && twice_appeared {
            count += 1;
        }

        repeats = false;
        twice_appeared = false;
    }

    count
}
