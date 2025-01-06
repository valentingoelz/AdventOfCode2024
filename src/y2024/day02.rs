pub fn solve(input: String) {
    let lines: Vec<&str> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut count = 0;

    for line in lines {
        // check if line is valid with any one number removed
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        if check_line(&numbers) {
            count += 1;
        } else {
            for i in 0..numbers.len() {
                // create new subline with one number removed
                let mut sub_line = numbers.clone();
                sub_line.remove(i);
                println!("{:?}", sub_line);
                if check_line(&sub_line) {
                    count += 1;
                    //println!("Valid line: {}", sub_line);
                    break;
                }
            }
        }
    }
    println!("Solution: {}", count);
}

fn check_line(numbers: &Vec<i32>) -> bool {
    // check if all numbers increase or decrease
    let mut increasing = true;
    let mut decreasing = true;
    for i in 1..numbers.len() {
        if numbers[i] < numbers[i - 1] {
            increasing = false;
        }
        if numbers[i] > numbers[i - 1] {
            decreasing = false;
        }
    }

    // check if distance between numbers is between 1 and 3, if not, continue to next line
    if increasing || decreasing {
        let possible_distances = [1, 2, 3];
        let mut valid = true;
        for i in 1..numbers.len() {
            if !possible_distances.contains(&(numbers[i] - numbers[i - 1]).abs()) {
                valid = false;
            }
        }
        if valid {
            return true;
        }
    }
    return false;
}
