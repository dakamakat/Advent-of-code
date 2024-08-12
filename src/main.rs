use days::day5::intern_work_second_iteration;

mod days;
mod utils;

fn main() {
    let input = utils::read_lines("./src/days/inputs/day5.txt");
    let result = intern_work_second_iteration(input);

    print!("{0}", result);
}
