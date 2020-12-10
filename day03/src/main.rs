use std::fs;

// starting from 0,0 moving +delta_x,+delta_y each iteration, count how many 'trees' (#) we encounter in the input
// the input wraps horizontally, ends at the bottom
fn check_slope(lines: &Vec<&str>, delta_x: usize, delta_y: usize) -> i32 {
    let mut trees_hit: i32 = 0;

    let mut x = 0;
    let mut y = 0;
    loop {
        let ch = lines[y].chars().nth(x % lines[y].len()).unwrap();
        if ch == '#' {
            trees_hit += 1;
        }

        x += delta_x;
        y += delta_y;

        if y >= lines.len() {
            break;
        }
    }

    return trees_hit;
}

// check slope +3,+1
fn part1(lines: &Vec<&str>) -> i32 {
    return check_slope(&lines, 3, 1);
}

// check slopes +1,+1, +3,+1, +5,+1, +7,+1, +1,+2
// multiply results
fn part2(lines: &Vec<&str>) -> i64 {
    let slope_results = vec![
        check_slope(&lines, 1, 1),
        check_slope(&lines, 3, 1),
        check_slope(&lines, 5, 1),
        check_slope(&lines, 7, 1),
        check_slope(&lines, 1, 2)
    ];

    println!("{} {} {} {} {}", slope_results[0], slope_results[1], slope_results[2], slope_results[3], slope_results[4]);
    return slope_results.iter().map(|x| *x as i64).product(); // need to cast to i64 to avoid overflow
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = input.lines().collect();

    println!("Part 1:");
    let part1_result = part1(&lines);
    println!("Trees hit: {}", part1_result);

    println!("Part 2:");
    let part2_result = part2(&lines);
    println!("Result: {}", part2_result);
}
