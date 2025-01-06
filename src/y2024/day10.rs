pub fn solve(input: String) {
    let grid = Grid::parse(&input);
    let (part1, part2) = grid.solve();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    starting_positions: Vec<(usize, usize)>,
    grid: Vec<Vec<usize>>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        let starting_positions = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &x)| x == 0)
                    .map(move |(j, _)| (i, j))
            })
            .collect();
        Self {
            width: grid[0].len(),
            height: grid.len(),
            starting_positions,
            grid,
        }
    }

    fn valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn get_adjacent(&self, x: usize, y: usize, current_value: usize) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();
        // no diagonals
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            // get self.grid
            if self.valid(new_x, new_y) {
                if self.grid[new_x as usize][new_y as usize] == current_value + 1 {
                    adjacent.push((new_x as usize, new_y as usize));
                }
            }
        }
        adjacent
    }

    fn solve(&self) -> (usize, usize) {
        let mut total_paths_2 = 0;
        let mut total_paths = 0;
        for position in self.starting_positions.iter() {
            let mut current_value = 0;
            let mut adjacent = vec![*position];
            while current_value < 9 {
                // find new adjacent and remove old ones
                let mut new_adjacent = Vec::new();
                for (x, y) in adjacent.iter() {
                    new_adjacent.extend(self.get_adjacent(*x, *y, current_value));
                }
                adjacent = new_adjacent;
                current_value += 1;
            }
            // filter for unique starting positions
            let unique_adjacent = adjacent.iter().collect::<std::collections::HashSet<_>>();
            total_paths += unique_adjacent.len();
            total_paths_2 += adjacent.len();
        }
        (total_paths, total_paths_2)
    }
}
