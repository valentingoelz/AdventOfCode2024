pub fn solve(input: String) {
    let input = input.split_once("\n\n").unwrap();

    let first_part = input.0;
    let second_part = input.1;

    let first_part = first_part
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|x| {
            let (x, y) = x.split_once("|").unwrap();
            return (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        })
        .collect::<Vec<_>>();

    let second_part = second_part
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|x| {
            x.split(",")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    let mut inactive: Vec<_> = Vec::new();

    for line in second_part {
        let mut active = true;
        for (x, y) in &first_part {
            if let (Some(x_idx), Some(y_idx)) = (
                line.iter().position(|&z| z == *x),
                line.iter().position(|&z| z == *y),
            ) {
                // check that x comes first
                if !(x_idx < y_idx) {
                    active = false;
                    break;
                }
            }
        }
        if active {
            let line_len = line.len();
            let middle_number = line[line_len / 2];
            sum += middle_number;
        } else {
            inactive.push(line);
        }
    }

    let mut sum2 = 0;
    for line in inactive {
        let mut new_line = line.clone();
        new_line.sort_by(|a, b| {
            for (x, y) in &first_part {
                if a == x && b == y {
                    return std::cmp::Ordering::Less;
                }
                if a == y && b == x {
                    return std::cmp::Ordering::Greater;
                }
            }
            return a.cmp(b);
        });
        println!("{:?}", new_line);
        let line_len = new_line.len();
        let middle_number = new_line[line_len / 2];
        sum2 += middle_number;
    }

    println!("Solution 1: {}", sum);
    println!("Solution 2: {}", sum2);
}
