fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let lines = input.lines().collect::<Vec<&str>>();

    let mut total = 0;

    for line in lines {
        total += line_value(line);
    }
    
    println!("Total: {}", total);
}

fn line_value(line: &str) -> u32 {
        let digits = line.chars().filter(|c| c.is_digit(10)).collect::<String>();

        if digits.len() == 0 {
            return 0;
        } else {
            if digits.len() == 1 {
                let digits = digits.repeat(2);
                return digits.parse::<u32>().unwrap();
            } else {
                let digits = digits.chars().take(1).collect::<String>()
                    + &digits.chars().rev().take(1).collect::<String>();
                return digits.parse::<u32>().unwrap();
            }
        }
}
