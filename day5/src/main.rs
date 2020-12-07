use std::fs;

struct SeatLocation {
    row: u8,
    seat: u8
}

// each pass consists of 7 chars for row + 3 chars for seat
// rows are 0-127, seats 0-8
// location of either is found via binary space partitioning, see below
fn get_pass_location(pass: &str) -> SeatLocation {
    let row_chars = &pass[0..7];
    let seat_chars = &pass[7..10];

    let mut row_min = 0;
    let mut row_max = 127;
    for ch in row_chars.chars() {
        if ch == 'F' {
            // F = Front = take lower half = clamp max to avg of min/max, round down
            row_max = (row_min + row_max) >> 1;
        } else if ch == 'B' {
            // B = Back = take upper half = clamp min to avg of min/max + 1
            row_min = ((row_min + row_max) >> 1) + 1;
        } else {
            panic!("invalid row char {} in pass {}", ch, pass);
        }
    }

    if row_min != row_max {
        panic!("could not arrive at row number for pass {}, min {}, max {}", pass, row_min, row_max);
    }

    let mut seat_min = 0;
    let mut seat_max = 7;
    for ch in seat_chars.chars() {
        if ch == 'L' {
            // L = Left = take lower half = clamp max to avg of min/max, round_down
            seat_max = (seat_min + seat_max) >> 1;
        } else if ch == 'R' {
            // R = Right = take upper half = clamp min to avg of min/max + 1
            seat_min = ((seat_min + seat_max) >> 1) + 1;
        } else {
            panic!("invalid seat char {} in pass {}", ch, pass);
        }
    }

    if seat_min != seat_max {
        panic!("could not arrive at seat number for pass {}, min {}, max {}", pass, seat_min, seat_max);
    }

    return SeatLocation {
        row: row_min,
        seat: seat_min
    }
}

// seat id = row * 8 + column (seat)
fn get_seat_id(location: &SeatLocation) -> u32 {
    return location.row as u32 * 8 + location.seat as u32;
}

// return highest seat id
fn part1(passes: &Vec<&str>) -> u32 {
    let mut result: u32 = 0;

    for pass in passes {
        let location = get_pass_location(&pass);
        let seat_id = get_seat_id(&location);
        if seat_id > result {
            result = seat_id;
        }
    }

    return result;
}

// return missing seat id between 2 other seat ids
fn part2(passes: &Vec<&str>) -> u32 {
    let mut seat_ids: Vec<u32> = vec![];

    for pass in passes {
        let location = get_pass_location(&pass);
        let seat_id = get_seat_id(&location);
        seat_ids.push(seat_id);
    }

    seat_ids.sort();

    let mut last_id = 0;
    for seat_id in seat_ids {
        if last_id != 0 && seat_id - last_id > 1 {
            return seat_id - 1;
        }
        last_id = seat_id;
    }

    panic!("no missing seat id found");
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");
    
    let passes = input.lines().collect();
    
    println!("Part 1:");
    let part1_result = part1(&passes);
    println!("Highest seat ID: {}", part1_result);
    
    println!("Part 2:");
    let part2_result = part2(&passes);
    println!("Your ID: {}", part2_result);
}
