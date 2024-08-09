use days::day4::mine_hash;

mod days;
mod utils;

fn main() {
    let input = utils::read_signle("./src/days/inputs/day4.txt".to_string());
    let result = mine_hash(input);

    print!("{0}", result);
}
