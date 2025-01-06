pub fn solve(input: String) {
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    // convert input string that only contains numbers to vector of integers
    let mut numbers = vec![];
    for line in lines {
        for character in line.chars() {
            if character.is_digit(10) {
                numbers.push(character.to_digit(10).unwrap() as i32);
            }
        }
        println!("{:?}", numbers);
    }
    let mut new_numbers = vec![];

    for i in 0..numbers.len() {
        if i % 2 == 0 {
            new_numbers.push(numbers[i]);
        } else {
            for _ in 0..numbers[i] {
                new_numbers.push(-1);
            }
        }
    }
    println!("{:?}", new_numbers);
}
