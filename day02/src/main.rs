use std::fs;
use regex::Regex;

// policy_char must appear between min_chars and max_chars times in password
fn part1_password_is_valid(min_chars: i32, max_chars: i32, policy_char: &char, password: &str) -> bool {
    let mut chars = 0;

    for c in password.chars() {
        if c == *policy_char {
            chars += 1;
            if chars > max_chars {
                return false;
            }
        }
    }

    if chars < min_chars {
        return false;
    }

    return true;
}

fn part1(lines: &Vec<&str>) -> i32 {
    let regex = Regex::new(r"^(\d+)-(\d+) (\w): (.+)$").unwrap();
    let mut valid_passwords = 0;

    for line in lines {
        let parts = regex.captures(line);
        if parts.is_none() {
            panic!("invalid input: {}", line)
        }

        let unwrapped = parts.unwrap();
        let min_chars = unwrapped[1].parse::<i32>().unwrap();
        let max_chars = unwrapped[2].parse::<i32>().unwrap();
        let policy_char = unwrapped[3].chars().nth(0).unwrap();
        let password = &unwrapped[4];

        if part1_password_is_valid(min_chars, max_chars, &policy_char, password) {
            valid_passwords += 1;
        }
    }

    return valid_passwords;
}

// policy_char must appear in EITHER pos1 OR pos2, not both
fn part2_password_is_valid(pos1: i32, pos2: i32, policy_char: &char, password: &str) -> bool {
    let mut found_in_pos1 = false;

    for (i, c) in password.chars().enumerate() {
        if c == *policy_char {
            let pos = i as i32 + 1;
            if pos == pos1 {
                found_in_pos1 = true;
            } else if pos == pos2 {
                // As long as we didn't already find the char in pos1, this returns true
                return ! found_in_pos1;
            }
        }
    }

    // If we found the char in pos1 but not pos2, success
    return found_in_pos1;
}

fn part2(lines: &Vec<&str>) -> i32 {
    let regex = Regex::new(r"^(\d+)-(\d+) (\w): (.+)$").unwrap();
    let mut valid_passwords = 0;

    for line in lines {
        let parts = regex.captures(line);
        if parts.is_none() {
            panic!("invalid input: {}", line)
        }

        let unwrapped = parts.unwrap();
        let pos1 = unwrapped[1].parse::<i32>().unwrap();
        let pos2 = unwrapped[2].parse::<i32>().unwrap();
        let policy_char = unwrapped[3].chars().nth(0).unwrap();
        let password = &unwrapped[4];

        if part2_password_is_valid(pos1, pos2, &policy_char, password) {
            valid_passwords += 1;
        }
    }

    return valid_passwords;
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = input.lines().collect();

    let part1_result = part1(&lines);
    println!("Part 1:");
    println!("Valid passwords: {}", part1_result);

    let part2_result = part2(&lines);
    println!("Part 2:");
    println!("Valid passwords: {}", part2_result);
}
