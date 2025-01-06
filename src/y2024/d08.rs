use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    antennas: HashMap<char, HashSet<Coordinate>>,
}

impl Grid {
    fn contains(&self, coord: &Coordinate) -> bool {
        coord.x >= 0 && coord.y >= 0 && coord.x < self.width as i32 && coord.y < self.height as i32
    }
}

pub fn solve(input: String) {
    let mut antennas = HashMap::new();
    input.trim().split("\n").enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert(HashSet::new())
                    .insert(Coordinate {
                        x: x as i32,
                        y: y as i32,
                    });
            }
        })
    });

    let grid = Grid {
        width: input.lines().next().unwrap().len(),
        height: input.lines().count(),
        antennas,
    };

    let mut positions_1 = HashSet::new();

    for (_, coords) in grid.antennas.iter() {
        // for each pair of antennas
        for coord1 in coords.iter() {
            for coord2 in coords.iter() {
                if coord1 == coord2 {
                    continue;
                }

                let dx = coord2.x - coord1.x;
                let dy = coord2.y - coord1.y;

                let coord = Coordinate {
                    x: coord2.x + dx,
                    y: coord2.y + dy,
                };
                if grid.contains(&coord) {
                    positions_1.insert(coord);
                }
            }
        }
    }

    let mut positions_2 = HashSet::new();

    for (_, coords) in grid.antennas.iter() {
        for coord1 in coords.iter() {
            for coord2 in coords.iter() {
                if coord1 == coord2 {
                    continue;
                }

                let dx = coord2.x - coord1.x;
                let dy = coord2.y - coord1.y;
                let mut i = 0;

                loop {
                    let coord = Coordinate {
                        x: coord2.x + i * dx,
                        y: coord2.y + i * dy,
                    };
                    if !grid.contains(&coord) {
                        break;
                    }
                    positions_2.insert(coord);
                    i += 1;
                }
            }
        }
    }

    println!("Level 1: {}", positions_1.len());
    println!("Level 2: {}", positions_2.len());
}
