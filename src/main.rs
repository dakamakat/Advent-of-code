use days::day3::{at_least_one, at_least_one_robo};

mod days;
mod utils;

fn main() {
    let input = utils::read_signle("./src/days/inputs/day3.txt".to_string());
    let result = at_least_one_robo(input);

    print!("{0}", result);
}
