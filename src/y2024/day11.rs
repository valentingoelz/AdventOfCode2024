use std::collections::HashMap;

pub fn solve(input: String) {
    let input = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let numbers = input[0]
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("Part 1: {}", solve_better(&numbers, 25));
    println!("Part 2: {}", solve_better(&numbers, 75))
}

fn solve_better(numbers: &Vec<i64>, blinks: usize) -> usize {
    let mut stones: HashMap<u64, usize> = numbers.iter().map(|s| (*s as u64, 1 as usize)).collect();
    for _ in 0..blinks {
        for (stone, n) in stones.drain().collect::<Vec<_>>() {
            let mut insert = |s| {
                stones.entry(s).and_modify(|x| *x += n).or_insert(n);
            };
            if stone == 0 {
                insert(1);
            } else {
                match (stone as f32).log10().floor() as u32 + 1 {
                    digits if digits % 2 == 0 => {
                        insert(stone / 10u64.pow(digits / 2));
                        insert(stone % 10u64.pow(digits / 2));
                    }
                    _ => insert(stone * 2024),
                }
            }
        }
    }
    stones.values().sum()
}
