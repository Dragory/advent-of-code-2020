#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

struct ContainedBag {
    bag: String,
    count: u8
}

struct Rule {
    bag: String,
    contained_bags: Vec<ContainedBag>
}

// parse a rule of the form "shiny gold bags contain 2 dark red bags."
fn parse_rule(rule: &str) -> Rule {
    lazy_static! {
        static ref RULE_RE: Regex = Regex::new(r"^(.+?) bags contain (.+?)$").unwrap();
        static ref RULE_CONTAIN_RE: Regex = Regex::new(r"(\d+) (.+?) bags?").unwrap();
    }

    let rule_parts = RULE_RE.captures(rule);
    if rule_parts.is_none() {
        panic!("Invalid rule: {}", rule);
    }

    let rule_parts_uw = rule_parts.unwrap();

    let parent_bag = rule_parts_uw[1].to_string();

    let mut contained_bags: Vec<ContainedBag> = vec![];
    let contained_bags_raw = &rule_parts_uw[2];
    let contained_bags_iter = RULE_CONTAIN_RE.captures_iter(contained_bags_raw);
    for contained_bag in contained_bags_iter {
        contained_bags.push(ContainedBag {
            bag: contained_bag[2].to_string(),
            count: contained_bag[1].parse::<u8>().unwrap()
        });
    }

    return Rule {
        bag: parent_bag,
        contained_bags
    };
}

// "How many bag colors can eventually contain at least one shiny gold bag?"
fn part1(rules: &Vec<&str>) -> u32 {
    // create a map of each bag color and their parents (bags they can be contained in)
    let mut bag_parents: HashMap<String, Vec<String>> = HashMap::new();

    for rule in rules {
        let parsed_rule = parse_rule(rule);
        for contained_bag in parsed_rule.contained_bags {
            match bag_parents.get_mut(&contained_bag.bag) {
                Some(x) => x.push(parsed_rule.bag.clone()),
                None => {
                    bag_parents.insert(contained_bag.bag, vec![parsed_rule.bag.clone()]);
                    ()
                }
            }
        }
    }

    // from that map, find how many parents the shiny gold bag has
    let parents = find_deep_parents(&bag_parents, "shiny gold", None);
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

// "How many individual bags are required inside your single shiny gold bag?"
fn part2(rules: &Vec<&str>) -> u32 {
    // build map of bag -> contained bags
    let mut bag_map: HashMap<String, Vec<ContainedBag>> = HashMap::new();

    for rule in rules {
        let parsed_rule = parse_rule(rule);
        bag_map.insert(parsed_rule.bag, parsed_rule.contained_bags);
    }

    // count how many bags are nested within a shiny gold bag
    return count_contained_bags(&bag_map, "shiny gold");
}

// recursively count how many contained/inner bags a specific bag has in the bag map
fn count_contained_bags(bag_map: &HashMap<String, Vec<ContainedBag>>, ref_bag: &str) -> u32 {
    let contained_bags = bag_map.get(ref_bag);
    if contained_bags.is_none() {
        return 0;
    }

    let mut count: u32 = 0;
    for contained_bag in contained_bags.unwrap() {
        count += contained_bag.count as u32;
        count += count_contained_bags(bag_map, &contained_bag.bag) * contained_bag.count as u32;
    }

    return count;
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Error reading input file");
    
    let rules = input.lines().collect();

    println!("Part 1:");
    let part1_result = part1(&rules);
    println!("Result: {}", part1_result);

    println!("Part 2:");
    let part2_result = part2(&rules);
    println!("Result: {}", part2_result);
}
