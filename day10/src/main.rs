// https://adventofcode.com/2020/day/10

use std::fs;

// "What is the number of 1-jolt differences multiplied by the number of 3-jolt differences?"
fn part1(adapters: &Vec<i32>) -> i32 {
    let mut diff1 = 0;
    let mut diff3 = 0;

    let mut prev: i32 = 0;
    for adapter in adapters {
        match adapter - prev {
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => ()
        };
        prev = *adapter;
    }

    return diff1 * diff3;
}

// "What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?"
// this would take extremely long to brute force, so split the adapters into smaller groups at the points where the next adapter is 3 jolts away,
// and then brute force the smaller groups. since 3 jolts is the maximum difference between 2 subsequent adapters, we know that the smaller groups
// can only join each other in one arrangement, and thus we can just multiply the number of arrangements in each group together.
fn part2(adapters: &Vec<i32>) -> i64 {
    let mut result: i64 = 1;

    let mut group = vec![0];
    for adapter in adapters {
        if *adapter > *group.last().unwrap() + 2 {
            let chain_count = find_chain_count(*group.first().unwrap(), &group[1..]);
            result *= chain_count;
            group = vec![];
        }

        group.push(*adapter);
    }

    if group.len() > 0 {
        result *= find_chain_count(*group.first().unwrap(), &group[1..]);
    }

    return result;
}

fn find_chain_count(head: i32, tail: &[i32]) -> i64 {
    if tail.len() == 0 {
        return 1;
    }

    let mut count: i64 = 0;
    for (i, adapter) in tail.iter().enumerate() {
        if *adapter > head + 3 {
            break;
        }

        count += find_chain_count(*adapter, &tail[(i + 1)..]);
    }

    return count;
}

// you have time to make some coffee
fn part2_bruteforce(adapters: &Vec<i32>) -> i64 {
    return find_chain_count(0, &adapters);
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");
    
    let mut adapters = input.lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    adapters.sort();
    
    // add device as last adapter
    adapters.push(adapters.iter().max().unwrap() + 3);

    println!("Part 1:");
    let part1_result = part1(&adapters);
    println!("Result: {}", part1_result);

    println!("Part 2:");
    let part2_result = part2(&adapters);
    println!("Result: {}", part2_result);
}
