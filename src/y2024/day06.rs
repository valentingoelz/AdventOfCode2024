use std::collections::HashSet;

// make enum Direction able to copy
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: String) {
    let input = create_readable_input(input);

    let input_part1 = solve_part1(input.clone());
    // count the number of X in the input
    let result_part1 = input_part1
        .iter()
        .map(|line| line.iter().filter(|tile| **tile == 'X').count())
        .sum::<usize>();

    // print current input and make sure new lines are printed below
    // for line in &input {
    //     println!("{}", line.iter().collect::<String>());
    // }

    println!("Solution part 1: {}", result_part1);

    println!("Solution part 2: {}", solve_part2(input.clone()));
}

fn find_current_position(input: &Vec<Vec<char>>) -> (i32, i32) {
    let (x, y) = input
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|tile| *tile == '^').map(|j| (i, j)))
        .expect("No start position found");
    (x as i32, y as i32)
}

fn is_valid_position(bounds: (i32, i32), position: (i32, i32)) -> bool {
    let (x, y) = position;
    let (x_bound, y_bound) = (bounds.0, bounds.1);

    let result = x >= 0 && x < x_bound && y >= 0 && y < y_bound;
    result
}

fn next_position(current_position: (i32, i32), direction: &Direction) -> (i32, i32) {
    let (i, j) = current_position;
    match direction {
        Direction::Up => (i - 1, j),
        Direction::Down => (i + 1, j),
        Direction::Left => (i, j - 1),
        Direction::Right => (i, j + 1),
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn solve_part1(mut input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // find start position
    let mut current_position = find_current_position(&input);
    let mut direction = Direction::Up;

    let bounds = (input.len() as i32, input[0].len() as i32);

    loop {
        input[current_position.0 as usize][current_position.1 as usize] = 'X';

        let (i, j) = current_position;
        let (x, y) = next_position(current_position, &direction);
        // if next position is Free, move there. Check that next position is not out of bound
        // check that next position is still in array

        if !is_valid_position(bounds, (x, y)) {
            return input;
        }

        let next_tile = input[x as usize][y as usize];
        match next_tile {
            '.' => {
                input[i as usize][j as usize] = 'X';
                current_position = (x, y);
            }
            'X' => current_position = (x, y),
            '#' => {
                // if next position is wall, turn right
                direction = turn_right(direction);
                current_position = next_position(current_position, &direction);
                if !is_valid_position(bounds, current_position) {
                    return input;
                }
            }
            _ => panic!("Invalid tile found: {}", next_tile),
        }
    }
}

fn solve_part2(mut input: Vec<Vec<char>>) -> i32 {
    // define iterator that will return next entry in the input (the inner part)
    let starting_position = find_current_position(&input);
    println!(
        "{:?}",
        input[starting_position.0 as usize][starting_position.1 as usize]
    );
    input[starting_position.0 as usize][starting_position.1 as usize] = '.';

    println!("{:?}", starting_position);
    let starting_direction = Direction::Up;
    let bounds = (input.len() as i32, input[0].len() as i32);

    let mut total_sum = 0;

    for i in 0..input.len() {
        eprintln!("{}", i);
        for j in 0..input[0].len() {
            let mut current_input = input.clone();

            if (i, j) == (starting_position.0 as usize, starting_position.1 as usize) {
                continue;
            }

            match current_input[i][j] {
                '#' => continue,
                '.' => current_input[i][j] = 'O',
                _ => continue,
            }
            let mut current_position = starting_position.clone();
            let mut direction = starting_direction.clone();

            let mut past_positions = HashSet::new();
            loop {
                // print current input and make sure new lines are printed below
                let (x, y) = next_position(current_position, &direction);

                // if next position is Free, move there. Check that next position is not out of bound
                // check that next position is still in array
                // println!("Next position: {:?}", (x, y));

                if !is_valid_position(bounds, (x, y)) {
                    break;
                }

                let next_tile = current_input[x as usize][y as usize];
                match next_tile {
                    '.' => {
                        current_position = (x, y);
                    }
                    '#' | 'O' => {
                        // if next position is wall, turn right
                        direction = turn_right(direction);
                        //current_position = next_position(current_position, &direction);
                        if !is_valid_position(bounds, current_position) {
                            break;
                        }
                    }
                    _ => panic!("Invalid tile found: {}", next_tile),
                }
                if past_positions.contains(&(current_position, direction)) {
                    total_sum += 1;
                    println!("{} {}", i, j);
                    break;
                }
                past_positions.insert((current_position, direction));
            }
        }
    }
    total_sum
}

fn create_readable_input(input: String) -> Vec<Vec<char>> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
