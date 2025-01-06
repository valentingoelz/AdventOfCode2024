use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Coordinate {
    fn in_front(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y.wrapping_sub(1),
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x.wrapping_sub(1),
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn is_outside(&self, width: usize, height: usize) -> bool {
        self.x < 0 || self.y < 0 || self.x >= width as i32 || self.y >= height as i32
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    obstacles: HashSet<Coordinate>,
    start_position: Coordinate,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let lines = input.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();
        let height = lines.len();
        let width = lines[0].len();
        let mut obstacles = HashSet::new();
        let mut start_position = None;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert(Coordinate {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    '^' => {
                        start_position = Some(Coordinate {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    '.' => {}
                    _ => panic!("Invalid character: {}", c),
                }
            }
        }

        Self {
            width,
            height,
            obstacles,
            start_position: start_position.unwrap(),
        }
    }
}

pub fn solve(input: String) {
    let grid = Grid::parse(&input);

    let mut visited = HashSet::new();
    let mut position = grid.start_position;
    let mut direction = Direction::Up;

    loop {
        visited.insert(position);
        let in_front = position.in_front(&direction);
        if grid.obstacles.contains(&in_front) {
            direction = direction.rotate_right();
        } else if in_front.is_outside(grid.width, grid.height) {
            break;
        } else {
            position = in_front;
        }
    }

    println!("Level 1: {}", visited.len());

    // this can get much faster by looking for rectangles or using caching but i don't care
    let mut num_obstacles_leading_to_endless_loop = 0;
    for y in 0..grid.height as i32 {
        eprintln!("y: {}", y);
        for x in 0..grid.width as i32 {
            if grid.start_position == (Coordinate { x, y }) {
                continue;
            }

            let position = Coordinate { x, y };
            let mut new_obstacles = grid.obstacles.clone();
            new_obstacles.insert(position);
            let new_grid = Grid {
                width: grid.width,
                height: grid.height,
                obstacles: new_obstacles,
                start_position: grid.start_position,
            };

            let mut visited_with_direction = HashSet::new();
            let mut position = new_grid.start_position;
            let mut direction = Direction::Up;
            loop {
                visited_with_direction.insert((position, direction));
                let in_front = position.in_front(&direction);
                if new_grid.obstacles.contains(&in_front) {
                    direction = direction.rotate_right();
                } else if in_front.is_outside(new_grid.width, new_grid.height) {
                    break;
                } else {
                    position = in_front;
                }
                if visited_with_direction.contains(&(position, direction)) {
                    println!("{} {}", y, x);
                    num_obstacles_leading_to_endless_loop += 1;
                    break;
                }
            }
        }
    }

    println!("Level 2: {}", num_obstacles_leading_to_endless_loop);
}
