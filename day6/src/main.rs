use std::fs;
use std::collections::HashSet;

// each group has lines of chars
// return the sum of the number of unique chars within a group for each group
fn part1(groups: &Vec<&str>) -> u32 {
    let mut sum_of_all_group_yesses: u32 = 0;

    for group in groups {
        let mut yesses = HashSet::new();
        for line in group.lines() {
            for ch in line.chars() {
                yesses.insert(ch);
            }
        }
        sum_of_all_group_yesses += yesses.len() as u32;
    }

    return sum_of_all_group_yesses;
}

// each group has lines of chars
// return the sum of the number of *shared* chars within *each line of a group* for each group
fn part2(groups: &Vec<&str>) -> u32 {
    let mut sum_of_all_group_shared_yesses: u32 = 0;

    for group in groups {
        let shared_yesses = group.lines().fold(None, |set: Option<HashSet<char>>, line| {
            match set {
                Some(mut set) => {
                    let line_chars = line.chars().collect::<Vec<char>>();
                    set.retain(|&ch| line_chars.contains(&ch)); // only retain chars that were already present in the set
                    return Some(set);
                }
                None => {
                    let mut set = HashSet::new();
                    for ch in line.chars() {
                        set.insert(ch);
                    }
                    return Some(set);
                }
            };
        });

        sum_of_all_group_shared_yesses += shared_yesses.unwrap().len() as u32;
    }

    return sum_of_all_group_shared_yesses;
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input file");
    
    let groups = input.trim().split("\n\n").collect::<Vec<_>>();

    println!("Part 1:");
    let part1_result = part1(&groups);
    println!("Sum of each group's yesses: {}", part1_result);

    println!("Part 2:");
    let part2_result = part2(&groups);
    println!("Sum of each group's shared yesses: {}", part2_result);
}
