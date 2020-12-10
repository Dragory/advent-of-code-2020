use std::fs;

fn part1(numbers: &Vec<i64>) -> i64 {
    'search: for num_i in 25..numbers.len() {
        for i in (num_i - 25)..num_i {
            for j in (i + 1)..num_i {
                if numbers[i] + numbers[j] == numbers[num_i] {
                    continue 'search;
                }
            }
        }
        return numbers[num_i];
    }

    panic!("No solution to part 1");
}

fn part2(numbers: &Vec<i64>, target: i64) -> i64 {
    for i in 0..numbers.len() {
        let mut sum: i64 = numbers[i];
        for j in (i + 1)..numbers.len() {
            sum += numbers[j];
            if sum == target {
                let min = numbers[i..=j].iter().min().unwrap();
                let max = numbers[i..=j].iter().max().unwrap();
                return min + max;
            }
        }
    }

    panic!("No solution to part 2");
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input file");
    
    let numbers = input.lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    
    println!("Part 1:");
    let part1_result = part1(&numbers);
    println!("Result: {}", part1_result);
    
    println!("Part 2:");
    let part2_result = part2(&numbers, part1_result);
    println!("Result: {}", part2_result);
}
