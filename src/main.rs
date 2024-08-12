use days::day5::intern_work;

mod days;
mod utils;

fn main() {
    let input = utils::read_lines("./src/days/inputs/day4.txt");
    let result = intern_work(input);

    print!("{0}", result);
}
