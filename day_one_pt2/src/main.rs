fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let lines = input.lines().collect::<Vec<&str>>();

    let mut total = 0;

    for line in lines {
        let new_line = line_to_digits(line);
        total += line_value(&new_line);
    }
    
    println!("Total: {}", total);
}


fn line_value(digits: &str) -> u32 {
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


fn line_to_digits(line: &str) -> String {

    let number_words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut number_string = String::new();
    let mut window = String::new();

    for c in line.chars() {
        if c.is_digit(10) {
            number_string.push(c);
            window.push(c);
        } else {
            window.push(c);
            for (i,word) in number_words.iter().enumerate() {
                if window.contains(word) {
                    number_string.push_str(&(i+1).to_string());
                    window = word.to_string();
                    window.remove(0);
                    break;
                }
            }
        }
    }

    number_string
}
