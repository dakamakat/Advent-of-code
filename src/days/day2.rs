#![allow(dead_code)]

use std::cmp::min;

pub fn calculate_feets(input: Vec<String>) -> i32 {
    let mut total = 0;
    for line in input {
        let slices: Vec<&str> = line.split('x').collect();

        let l: i32 = slices[0].parse().unwrap();
        let w: i32 = slices[1].parse().unwrap();
        let h: i32 = slices[2].parse().unwrap();

        let lw = l * w;
        let wh = w * h;
        let hl = h * l;

        let lowest = min(lw, min(wh, hl));

        let feets = 2 * lw + 2 * wh + 2 * hl;
        total += feets + lowest;
    }

    total
}

pub fn calculate_ribbon(input: Vec<String>) -> i32 {
    let mut total = 0;
    for line in input {
        let slices: Vec<&str> = line.split('x').collect();

        let l: i32 = slices[0].parse().unwrap();
        let w: i32 = slices[1].parse().unwrap();
        let h: i32 = slices[2].parse().unwrap();

        let mut sides = vec![l, w, h];
        sides.sort();

        let bow = l * w * h;
        let wrap = 2 * sides[0] + 2 * sides[1];

        total += wrap + bow;
    }

    total
}
