use std::collections::HashMap;

fn main() {

    // Read the input.txt file
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut total = 0;
    for line in input.lines() {
        total+= process_line(line);
    }
    println!("Total: {}", total);
}


fn process_line(line: &str) -> u32 {

    let mut map = HashMap::new();
    map.insert("red", 12);
    map.insert("green", 13);
    map.insert("blue", 14);

    let line_split = line.split(":").collect::<Vec<&str>>();
    let game = line_split[0].split_whitespace().collect::<Vec<&str>>()[1];
    let rounds = line_split[1].split(";").collect::<Vec<&str>>();

    let mut possible = true;

    for draw in &rounds {
        let draw_split = draw.split(",").collect::<Vec<&str>>();
        for cube in &draw_split {
            let cube_split = cube.split_whitespace().collect::<Vec<&str>>();
            let color = cube_split[1];
            let value = cube_split[0].parse::<i32>().unwrap();
            if value > map[color] {
                possible = false;
            }
        }
    }

    if possible {
        return game.parse::<u32>().unwrap();
    } else {
        return 0;
    }
}
