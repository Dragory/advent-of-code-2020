#[macro_use] extern crate lazy_static;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

// each passport contains key:value pairs
// verify each required field/key exists
fn part1(passports: &Vec<&str>) -> i32 {
    let mut valid_passports = 0;
    let re = Regex::new(r"(\S+):(\S+)").unwrap();
    let required_fields = vec![
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid"
    ];

    for passport in passports {
        let mut required_fields_found = 0;
        for matches in re.captures_iter(passport) {
            if required_fields.contains(&&matches[1]) {
                required_fields_found += 1;
            }
        }
        if required_fields_found >= required_fields.len() {
            valid_passports += 1;
        }
    }

    return valid_passports;
}

// each required field has specific validation requirements
// verify each required field/key exists and its value passes validation
fn part2(passports: &Vec<&str>) -> i32 {
    let mut valid_passports = 0;
    let re = Regex::new(r"(\S+):(\S+)").unwrap();
    
    type ValidationFn = fn(&str) -> bool;
    let mut required_fields: HashMap<&str, ValidationFn> = HashMap::new();
    required_fields.insert("byr", valid_byr);
    required_fields.insert("iyr", valid_iyr);
    required_fields.insert("eyr", valid_eyr);
    required_fields.insert("hgt", valid_hgt);
    required_fields.insert("hcl", valid_hcl);
    required_fields.insert("ecl", valid_ecl);
    required_fields.insert("pid", valid_pid);

    for passport in passports {
        let mut required_fields_found = 0;
        for matches in re.captures_iter(passport) {
            if required_fields.contains_key(&&matches[1]) {
                if required_fields[&matches[1]](&matches[2]) {
                    required_fields_found += 1;
                }
            }
        }
        if required_fields_found >= required_fields.len() {
            valid_passports += 1;
        }
    }

    return valid_passports;
}

// valid birth year = 4 digits, between 1920 and 2002
fn valid_byr(byr: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{4}$").unwrap();
    }

    if ! RE.is_match(byr) {
        return false;
    }

    let num = byr.parse::<i32>().unwrap();
    return num >= 1920 && num <= 2002;
}

// valid issue year: 4 digits, between 2010 and 2020
fn valid_iyr(iyr: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{4}$").unwrap();
    }

    if ! RE.is_match(iyr) {
        return false;
    }

    let num = iyr.parse::<i32>().unwrap();
    return num >= 2010 && num <= 2020;
}

// valid expiration year: 4 digits, between 2020 and 2030
fn valid_eyr(eyr: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{4}$").unwrap();
    }

    if ! RE.is_match(eyr) {
        return false;
    }

    let num = eyr.parse::<i32>().unwrap();
    return num >= 2020 && num <= 2030;
}

// valid height: number followed by cm or in, cm must be 150-193, in must be 59-76
fn valid_hgt(hgt: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }

    let matches = RE.captures(hgt);
    if matches.is_none() {
        return false;
    }

    let unwrapped = matches.unwrap();
    let num = unwrapped[1].parse::<i32>().unwrap();
    let unit = &unwrapped[2];

    if unit == "cm" {
        return num >= 150 && num <= 193;
    } else {
        return num >= 59 && num <= 76;
    }
}

// valid hair color: valid hex color, i.e. # followed by six characters 0-9, a-f
fn valid_hcl(hcl: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }

    return RE.is_match(hcl);
}

// valid eye color: one of VALID_EYE_COLORS
fn valid_ecl(ecl: &str) -> bool {
    lazy_static! {
        static ref VALID_EYE_COLORS: Vec<&'static str> = vec![
            "amb",
            "blu",
            "brn",
            "gry",
            "grn",
            "hzl",
            "oth"
        ];
    }

    return VALID_EYE_COLORS.contains(&ecl);
}

// valid passport id: 9 digit number
fn valid_pid(pid: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    return RE.is_match(pid);
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file");

    let passports = input.split("\n\n").collect::<Vec<_>>();

    println!("Part 1:");
    let part1_result = part1(&passports);
    println!("Valid passports: {}", part1_result);

    println!("Part 2:");
    let part2_result = part2(&passports);
    println!("Valid passports: {}", part2_result);
}
