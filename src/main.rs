use days::day6::lights;

mod days;
mod utils;

fn main() {
    let input = utils::read_lines("./src/days/inputs/day6.txt");
    let result = lights(input);

    print!("{0}", result);
}
