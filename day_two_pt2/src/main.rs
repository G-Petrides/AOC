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
    map.insert("red", 0);
    map.insert("green", 0);
    map.insert("blue", 0);

    let line_split = line.split(":").collect::<Vec<&str>>();
    let rounds = line_split[1].split(";").collect::<Vec<&str>>();

    for draw in &rounds {
        let draw_split = draw.split(",").collect::<Vec<&str>>();
        for cube in &draw_split {
            let cube_split = cube.split_whitespace().collect::<Vec<&str>>();
            let color = cube_split[1];
            let value = cube_split[0].parse::<i32>().unwrap();
            if value > map[color] {
                map.insert(color, value);
            }
        }
    }

    return map["red"] as u32 * map["green"] as u32 * map["blue"] as u32;
}
