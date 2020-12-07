use std::fs;

fn part1(lines: &Vec<&str>) -> Option<(i32, i32)> {
    let count = lines.len();

    for i in 0..(count-1) {
        let num1 = (&lines[i]).parse::<i32>().unwrap();
        for j in (i+1)..count {
            let num2 = lines[j].parse::<i32>().unwrap();
            if num1 + num2 == 2020 {
                return Some((num1, num2));
            }
        }
    }

    None
}

fn part2(lines: &Vec<&str>) -> Option<(i32, i32, i32)> {
    let count = lines.len();

    for i in 0..(count-2) {
        let num1 = (&lines[i]).parse::<i32>().unwrap();
        for j in (i+1)..count {
            let num2 = lines[j].parse::<i32>().unwrap();
            for k in (i+2)..count {
                let num3 = lines[k].parse::<i32>().unwrap();

                if num1 + num2 + num3 == 2020 {
                    return Some((num1, num2, num3));
                }
            }
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = input.lines().collect();

    let part1_result = part1(&lines);

    println!("Part 1:");
    match part1_result {
        Some(r) => {
            println!("{} * {} = {}", r.0, r.1, r.0 * r.1);
        }
        None => {
            println!("No two numbers that sum to 2020 found");
        }
    }

    let part2_result = part2(&lines);

    println!("Part 2:");
    match part2_result {
        Some(r) => {
            println!("{} * {} * {} = {}", r.0, r.1, r.2, r.0 * r.1 * r.2);
        }
        None => {
            println!("No three numbers that sum to 2020 found");
        }
    }
}
