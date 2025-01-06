use regex::Regex;

pub fn solve(input: String) {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();
    let mut sum = 0;
    let mut active = true;
    let iterator = re.find_iter(&input);
    for x in iterator {
        let mul = x.as_str();
        if mul == "do()" {
            active = true;
            continue;
        } else if mul == "don't()" {
            active = false;
            continue;
        } else {
            if !active {
                continue;
            }
        }
        let mul = mul.strip_prefix("mul(").unwrap();
        let mul = mul.strip_suffix(")").unwrap();
        let mut split = mul.split(",");
        let a = split.next().unwrap().parse::<i32>().unwrap();
        let b = split.next().unwrap().parse::<i32>().unwrap();
        sum += a * b;
    }
    println!("Solution: {}", sum);
}
