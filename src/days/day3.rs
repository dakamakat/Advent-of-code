pub fn at_least_one(input: String) -> i32 {
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;

    let mut field = [[0; WIDTH]; HEIGHT];

    let mut position = Position {
        width: 250,
        height: 250,
    };

    for it in input.chars() {
        place(
            it,
            position.height,
            position.width,
            &mut field,
            &mut position,
        )
    }

    let mut sum = 0;

    for arr in field {
        sum += arr.iter().sum::<i32>();
    }

    sum + 1
}

fn place(
    chr: char,
    rank: usize,
    index: usize,
    arr: &mut [[i32; 500]; 500],
    position: &mut Position,
) {
    if chr == '^' {
        let new_rank: usize = rank - 1;
        arr[new_rank][index] = 1;
        position.height = new_rank
    } else if chr == 'v' {
        let new_rank: usize = rank + 1;
        arr[new_rank][index] = 1;
        position.height = new_rank
    } else if chr == '<' {
        let new_index: usize = index - 1;
        arr[rank][new_index] = 1;
        position.width = new_index;
    } else if chr == '>' {
        let new_index: usize = index + 1;
        arr[rank][index + 1] = 1;
        position.width = new_index;
    }
}

struct Position {
    width: usize,
    height: usize,
}
