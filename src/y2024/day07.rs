pub fn solve(input: String) {
    let input = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let lines = input
        .iter()
        .map(|line| {
            let line = line.split_once(":").unwrap();
            let result = (line.0).parse::<i64>().unwrap();
            let values = line
                .1
                .split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (result, values)
        })
        .collect::<Vec<_>>();

    solve_part1(&lines);
    solve_part2(&lines);
}

fn solve_part1(lines: &Vec<(i64, Vec<i64>)>) {
    let mut sum = 0;
    for (result, values) in lines {
        let operators = get_list_of_operators(&values);
        for operator in operators {
            let mut current_result = values[0];
            for i in 0..values.len() - 1 {
                current_result = match operator[i] {
                    Operator::Add => current_result + values[i + 1],
                    Operator::Multiply => current_result * values[i + 1],
                    _ => {
                        unreachable!()
                    }
                };
            }
            if &current_result == result {
                sum += current_result;
                break;
            }
        }
    }
    println!("Solution part 1: {}", sum);
}

fn get_list_of_operators(values: &Vec<i64>) -> Vec<Vec<Operator>> {
    let n = values.len() - 1;
    let mut operators: Vec<Vec<Operator>> = vec![];
    for i in 0..2usize.pow(n as u32) {
        let mut current = i;
        let mut current_operators = vec![];
        for _ in 0..n {
            let operator = match current % 2 {
                0 => Operator::Add,
                1 => Operator::Multiply,
                _ => unreachable!(),
            };
            current_operators.push(operator);
            current /= 2;
        }
        operators.push(current_operators);
    }
    operators
}

fn solve_part2(lines: &Vec<(i64, Vec<i64>)>) {
    let mut sum = 0;
    for (result, values) in lines {
        let operators = get_list_of_three_operators(&values);
        for operator in operators {
            let mut current_result = values[0];
            for i in 0..values.len() - 1 {
                current_result = match operator[i] {
                    Operator::Add => current_result + values[i + 1],
                    Operator::Multiply => current_result * values[i + 1],
                    Operator::Concatenate => {
                        let current = current_result;
                        let next = values[i + 1];
                        // concatenate current and next number like two strings
                        let s = current.to_string() + &next.to_string();
                        s.parse::<i64>().unwrap()
                    }
                };
            }
            if &current_result == result {
                sum += current_result;
                break;
            }
        }
    }
    println!("Solution part 2: {}", sum);
}

fn get_list_of_three_operators(values: &Vec<i64>) -> Vec<Vec<Operator>> {
    let n = values.len() - 1;
    let mut operators: Vec<Vec<Operator>> = vec![];
    for i in 0..3usize.pow(n as u32) {
        let mut current = i;
        let mut current_operators = vec![];
        for _ in 0..n {
            let operator = match current % 3 {
                0 => Operator::Add,
                1 => Operator::Multiply,
                2 => Operator::Concatenate,
                _ => unreachable!(),
            };
            current_operators.push(operator);
            current /= 3;
        }
        operators.push(current_operators);
    }
    operators
}

enum Operator {
    Add,
    Multiply,
    Concatenate,
}
