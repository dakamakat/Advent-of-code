use days::day2::calculate_feets;

mod days;
mod utils;

fn main() {
    let input = utils::read_lines("./src/days/inputs/day2.txt");
    let result = calculate_feets(input);

    print!("{0}", result);
}
