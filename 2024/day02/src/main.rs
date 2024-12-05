use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    // Calculate part 1 answer
    let answer1 = process_part1(input.as_str());
    println!("Answer part 1: {}", answer1);

    // Calculate part 2 answer
    let answer2 = process_part2(input.as_str());
    println!("Answer part 2: {}", answer2);
}

fn process_part1(input: &str) -> u32 {
    let mut count = 0u32;

    for line in input.lines() {
        let line = line.split_whitespace();
        let vals: Vec<i32> = line.map(|n| n.parse::<i32>().unwrap()).collect();
        if check_safety(&vals) {
            count += 1;
        }
    }

    count
}

fn process_part2(input: &str) -> u32 {
    let mut count = 0u32;

    for line in input.lines() {
        let line = line.split_whitespace();
        let mut vals: Vec<i32> = line.map(|n| n.parse::<i32>().unwrap()).collect();
        let mut rem_idx = 0;
        let mut rem_val: Vec<i32> = Vec::new();

        loop {
            if check_safety(&vals) {
                count += 1;
                break;
            }
            if !rem_val.is_empty() {
                vals.insert(rem_idx, rem_val.pop().unwrap());
                rem_idx += 1;
            }
            if rem_idx < vals.len() {
                rem_val.push(vals.remove(rem_idx));
            } else {
                break;
            }
        }
    }

    count

}

fn check_safety(vals: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = vals.windows(2).map(|w| w[1]-w[0]).collect();
    diffs.windows(2).all(|diff|
        (1..=3).contains(&diff[0].abs()) &&
        (1..=3).contains(&diff[1].abs()) &&
        diff[0].signum() == diff[1].signum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = 
        "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1 
        1 3 6 7 9";

        assert_eq!(2, process_part1(input));
    }

    #[test]
    fn test_part2() {
        let input = 
        "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1 
        1 3 6 7 9";

        assert_eq!(4, process_part2(input));
    }
}