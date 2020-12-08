#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

// "How many bag colors can eventually contain at least one shiny gold bag?"
fn part1(rules: &Vec<&str>) -> u32 {
    lazy_static! {
        static ref RULE_RE: Regex = Regex::new(r"^(.+?) bags contain (.+?)$").unwrap();
        static ref RULE_CONTAIN_RE: Regex = Regex::new(r"(\d+) (.+?) bags?").unwrap();
    }

    // create a map of each bag color that has parents (is contained within another bag)
    let mut bag_parents: HashMap<String, Vec<String>> = HashMap::new();

    for rule in rules {
        let rule_parts = RULE_RE.captures(rule);
        if rule_parts.is_none() {
            panic!("Invalid rule: {}", rule);
        }

        let rule_parts_uw = rule_parts.unwrap();
        let parent_bag = rule_parts_uw[1].to_string();
        let contained_bags_raw = &rule_parts_uw[2];

        let contained_bags_iter = RULE_CONTAIN_RE.captures_iter(contained_bags_raw);
        for contained_bag in contained_bags_iter {
            let contained_bag_name = contained_bag[2].to_string();
            match bag_parents.get_mut(&contained_bag_name) {
                Some(x) => x.push(parent_bag.clone()),
                None => {
                    bag_parents.insert(contained_bag_name, vec![parent_bag.clone()]);
                    ()
                }
            }
        }
    }

    let foo = bag_parents.get("shiny gold").unwrap();
    println!("shiny gold direct parents: {}", foo.join(", "));

    let parents = find_deep_parents(&bag_parents, "shiny gold", None);
    let parent_names = parents.iter()
        .map(|x| String::from(x.clone()))
        .collect::<Vec<_>>()
        .join(", ");

    println!("parents: {}", parent_names);
    return parents.len() as u32;
}

// from a map of each bag and their parents, find how many "eventual" or "deep" parents the specified bag has
fn find_deep_parents<'a>(bag_parents: &'a HashMap<String, Vec<String>>, ref_bag: &str, found_parents: Option<HashSet<&'a String>>) -> HashSet<&'a String> {
    let mut real_found_parents = match found_parents {
        Some(x) => x,
        None => HashSet::new()
    };

    let root = bag_parents.get(ref_bag);
    if root.is_none() {
        return real_found_parents;
    }

    for parent in root.unwrap() {
        if real_found_parents.contains(parent) {
            continue;
        }

        real_found_parents.insert(parent);
        real_found_parents = find_deep_parents(bag_parents, parent, Some(real_found_parents));
    }

    return real_found_parents;
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Error reading input file");
    
    let rules = input.lines().collect();

    println!("Part 1:");
    let part1_result = part1(&rules);
    println!("Result: {}", part1_result);
}
