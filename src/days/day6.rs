#![allow(dead_code)]

pub fn lights(input: Vec<String>) -> i32 {
    let mut state = [[false; 1000]; 1000];

    for command in input {
        if let Some(through_index) = command.find("through") {
            let (first_part, second_part) = command.split_at(through_index);

            let first_coords: Vec<&str> = first_part
                .split_whitespace()
                .last()
                .unwrap()
                .split(',')
                .collect();

            let second_coords: Vec<&str> = second_part
                .split_whitespace()
                .last()
                .unwrap()
                .split(',')
                .collect();

            let x1 = first_coords[0].parse::<i32>().unwrap();
            let y1 = first_coords[1].parse::<i32>().unwrap();
            let x2 = second_coords[0].parse::<i32>().unwrap();
            let y2 = second_coords[1].parse::<i32>().unwrap();

            if command.starts_with("turn on") {
                set_light(&mut state, x1, y1, x2, y2, true)
            } else if command.starts_with("turn off") {
                set_light(&mut state, x1, y1, x2, y2, false)
            } else {
                set_light_toggle(&mut state, x1, y1, x2, y2)
            }
        }
    }

    let lights = state.iter().flatten().filter(|&&light| light).count() as i32;

    lights
}

pub fn brightness(input: Vec<String>) -> i32 {
    let mut state = [[0; 1000]; 1000];

    for command in input {
        if let Some(through_index) = command.find("through") {
            let (first_part, second_part) = command.split_at(through_index);

            let first_coords: Vec<&str> = first_part
                .split_whitespace()
                .last()
                .unwrap()
                .split(',')
                .collect();

            let second_coords: Vec<&str> = second_part
                .split_whitespace()
                .last()
                .unwrap()
                .split(',')
                .collect();

            let x1 = first_coords[0].parse::<i32>().unwrap();
            let y1 = first_coords[1].parse::<i32>().unwrap();
            let x2 = second_coords[0].parse::<i32>().unwrap();
            let y2 = second_coords[1].parse::<i32>().unwrap();

            if command.starts_with("turn on") {
                set_brightness(&mut state, x1, y1, x2, y2, 1)
            } else if command.starts_with("turn off") {
                set_brightness(&mut state, x1, y1, x2, y2, -1)
            } else {
                set_brightness_toggle(&mut state, x1, y1, x2, y2)
            }
        }
    }

    state.iter().flatten().sum()
}

fn set_brightness(
    state: &mut [[i32; 1000]; 1000],
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    brightness: i32,
) {
    for i in y1..=y2 {
        for j in x1..=x2 {
            state[i as usize][j as usize] += brightness;

            if state[i as usize][j as usize] < 0 {
                state[i as usize][j as usize] = 0
            }
        }
    }
}

fn set_brightness_toggle(state: &mut [[i32; 1000]; 1000], x1: i32, y1: i32, x2: i32, y2: i32) {
    for i in y1..=y2 {
        for j in x1..=x2 {
            state[i as usize][j as usize] += 2;
        }
    }
}

fn set_light(state: &mut [[bool; 1000]; 1000], x1: i32, y1: i32, x2: i32, y2: i32, enable: bool) {
    for i in y1..=y2 {
        for j in x1..=x2 {
            state[i as usize][j as usize] = enable;
        }
    }
}

fn set_light_toggle(state: &mut [[bool; 1000]; 1000], x1: i32, y1: i32, x2: i32, y2: i32) {
    for i in y1..=y2 {
        for j in x1..=x2 {
            state[i as usize][j as usize] = !state[i as usize][j as usize];
        }
    }
}
