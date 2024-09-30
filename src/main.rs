use days::day6::{brightness, lights};

mod days;
mod utils;

fn main() {
    let input = utils::read_lines("./src/days/inputs/day6.txt");
    let result = brightness(input);

    print!("{0}", result);
}
