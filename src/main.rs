use days::day2::{calculate_feets, calculate_ribbon};

mod days;
mod utils;

fn main() {
    let input = utils::read_lines("./src/days/inputs/day2.txt");
    let result = calculate_ribbon(input);

    print!("{0}", result);
}
