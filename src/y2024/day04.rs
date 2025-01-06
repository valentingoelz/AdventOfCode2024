use std::collections::vec_deque;

pub fn solve(input: String) {
    // split input
    let x = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let horizontal_length = x[0].len();
    let vertical_length = x.len();

    let mut xmas_counter = 0;

    // horizontal forward + backward
    for i in 0..vertical_length {
        let mut last_four = vec_deque::VecDeque::new();
        for j in 0..horizontal_length {
            last_four.push_back(x[i][j]);
            if last_four.len() > 4 {
                last_four.pop_front();
            }
            if last_four == ['X', 'M', 'A', 'S'] {
                xmas_counter += 1;
            }
            if last_four == ['S', 'A', 'M', 'X'] {
                xmas_counter += 1;
            }
        }
    }

    // vertical forward + backward
    for i in 0..horizontal_length {
        let mut last_four = vec_deque::VecDeque::new();
        for j in 0..vertical_length {
            last_four.push_back(x[j][i]);
            if last_four.len() > 4 {
                last_four.pop_front();
            }
            if last_four == ['X', 'M', 'A', 'S'] {
                xmas_counter += 1;
            }
            if last_four == ['S', 'A', 'M', 'X'] {
                xmas_counter += 1;
            }
        }
    }

    // diagonal forward + backward top left to bottom right
    for (start_point_x, start_point_y) in (0..horizontal_length)
        .map(|x| (x, 0))
        .chain((1..vertical_length).map(|x| (0, x)))
    {
        let mut last_four = vec_deque::VecDeque::new();
        let mut i = start_point_x;
        let mut j = start_point_y;
        while i < horizontal_length && j < vertical_length {
            last_four.push_back(x[j][i]);
            if last_four.len() > 4 {
                last_four.pop_front();
            }
            if last_four == ['X', 'M', 'A', 'S'] {
                xmas_counter += 1;
            }
            if last_four == ['S', 'A', 'M', 'X'] {
                xmas_counter += 1;
            }
            i += 1;
            j += 1;
        }
    }

    // diagonal forward + backward top right to bottom left
    for (start_point_x, start_point_y) in (0..horizontal_length)
        .map(|x| (x, 0))
        .chain((1..vertical_length).map(|x| (horizontal_length - 1, x)))
    {
        let mut last_four = vec_deque::VecDeque::new();
        let mut i = start_point_x;
        let mut j = start_point_y;
        for _ in 0..=horizontal_length {
            last_four.push_back(x[j][i]);
            if last_four.len() > 4 {
                last_four.pop_front();
            }
            if last_four == ['X', 'M', 'A', 'S'] {
                xmas_counter += 1;
            }
            if last_four == ['S', 'A', 'M', 'X'] {
                xmas_counter += 1;
            }
            if i == 0 || j == vertical_length - 1 {
                break;
            }
            i -= 1;
            j += 1;
        }
    }

    println!("XMAS Counter: {}", xmas_counter);

    println!("Horizontal Length: {}", horizontal_length);
    println!("Vertical Length: {}", vertical_length);

    // find all positions with letter A
    let mut a_positions = vec![];
    for i in 1..vertical_length - 1 {
        for j in 1..horizontal_length - 1 {
            if x[i][j] == 'A' {
                a_positions.push((i, j));
            }
        }
    }
    println!("A Positions: {:?}", a_positions);

    // for each position, check if there are M and S one position to the
    let mut x_mas_counter = 0;
    for (i, j) in a_positions {
        let top_left = x[i - 1][j - 1];
        let top_right = x[i - 1][j + 1];
        let bottom_left = x[i + 1][j - 1];
        let bottom_right = x[i + 1][j + 1];

        // diagonals need to be M or S, they cant be the same
        if (top_left == 'M' || top_left == 'S') && (bottom_right == 'M' || bottom_right == 'S') {
            if top_left != bottom_right {
                if (top_right == 'M' || top_right == 'S')
                    && (bottom_left == 'M' || bottom_left == 'S')
                {
                    if top_right != bottom_left {
                        x_mas_counter += 1;
                    }
                }
            }
        }
    }
    println!("X-MAS Counter: {}", x_mas_counter);
}
