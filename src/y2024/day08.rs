use std::collections::HashSet;

pub fn solve(input: String) {
    let lines = input.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();
    let mut grid = Grid::parse(&lines);
    grid.solve();
    println!("Part 1: {}", grid.signal_positions_1.len());
    println!("Part 2: {}", grid.signal_positions_2.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Antenna {
    column: i32,
    row: i32,
    signature: char,
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    antennas: HashSet<Antenna>,
    signatures: HashSet<char>,
    signal_positions_1: HashSet<(i32, i32)>,
    signal_positions_2: HashSet<(i32, i32)>,
}

impl Grid {
    fn parse(input: &Vec<&str>) -> Self {
        let mut antennas = HashSet::new();
        for (i, line) in input.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => continue,
                    _ => {
                        antennas.insert(Antenna {
                            column: j as i32,
                            row: i as i32,
                            signature: c,
                        });
                    }
                }
            }
        }
        let mut signatures = HashSet::new();
        for antenna in &antennas {
            signatures.insert(antenna.signature);
        }

        Self {
            width: input[0].len(),
            height: input.len(),
            antennas,
            signatures,
            signal_positions_1: HashSet::new(),
            signal_positions_2: HashSet::new(),
        }
    }

    fn valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn get_antenna_pairs(&self, signature: char) -> Vec<(Antenna, Antenna)> {
        let antennas: Vec<&Antenna> = self
            .antennas
            .iter()
            .filter(|antenna| antenna.signature == signature)
            .collect();

        let antennas: Vec<Antenna> = antennas.iter().map(|&x| *x).collect();
        let mut pairs = vec![];
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                pairs.push((antennas[i], antennas[j]));
            }
        }
        pairs
    }

    fn get_direction(&self, antenna1: Antenna, antenna2: Antenna) -> (i32, i32) {
        let dx = antenna2.column - antenna1.column;
        let dy = antenna2.row - antenna1.row;
        (dx, dy)
    }

    fn get_signal_coordinates(&self, antenna1: Antenna, antenna2: Antenna) -> Vec<(i32, i32)> {
        let (dx, dy) = self.get_direction(antenna1, antenna2);
        let signal1 = (antenna1.column - dx, antenna1.row - dy);
        let signal2 = (antenna2.column + dx, antenna2.row + dy);
        vec![signal1, signal2]
    }

    fn get_signal_coordinates_2(&self, antenna1: Antenna, antenna2: Antenna) -> Vec<(i32, i32)> {
        let mut signals = vec![];
        let (dx, dy) = self.get_direction(antenna1, antenna2);
        signals.push((antenna1.column, antenna1.row));
        signals.push((antenna2.column, antenna2.row));

        let mut direction1 = (antenna1.column - dx, antenna1.row - dy);
        while self.valid(direction1.0, direction1.1) {
            signals.push(direction1);
            direction1 = (direction1.0 - dx, direction1.1 - dy);
        }

        let mut direction2 = (antenna1.column + dx, antenna1.row + dy);
        while self.valid(direction2.0, direction2.1) {
            signals.push(direction2);
            direction2 = (direction2.0 + dx, direction2.1 + dy);
        }
        signals
    }

    fn solve(&mut self) {
        for signature in self.signatures.clone() {
            let pairs = self.get_antenna_pairs(signature);
            for (antenna1, antenna2) in pairs {
                let signal_coordinates_1 = self.get_signal_coordinates(antenna1, antenna2);
                let signal_coordinates_2 = self.get_signal_coordinates_2(antenna1, antenna2);
                for (x, y) in signal_coordinates_1 {
                    if self.valid(x, y) {
                        self.signal_positions_1.insert((x, y));
                    }
                }
                for (x, y) in signal_coordinates_2 {
                    if self.valid(x, y) {
                        self.signal_positions_2.insert((x, y));
                    }
                }
            }
        }
    }
}
