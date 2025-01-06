use std::vec;

pub fn solve(input: String) {
    let processed = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    // first number in line goes to left vector, second number goes to right vector
    let mut left = vec![];
    let mut right = vec![];

    for line in processed {
        // get first 5 characters and convert them to an int
        let left_number = line[0..5].parse::<i32>().unwrap();
        // get last 5 characters and convert them to an int
        let right_number = line[8..13].parse::<i32>().unwrap();
        left.push(left_number);
        right.push(right_number);
    }
    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }

    println!("Solution: {}", sum);

    // calculate for each number in the left array how often it appears in the right array

    let mut similarity_score = 0;

    for i in 0..left.len() {
        let count = right.iter().filter(|&&x| x == left[i]).count();
        similarity_score += count as i32 * left[i];
    }

    println!("Solution: {}", similarity_score);
}
