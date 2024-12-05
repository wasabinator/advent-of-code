use std::{collections::HashMap, fs::{self}, iter::zip};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    // Calculate part 1 answer
    let answer1 = process_part1(input.as_str()).unwrap();
    println!("Answer for part 1: {}", answer1);

    // Calculate part 2 answer
    let answer2 = process_part2(input.as_str()).unwrap();
    println!("Answer for part 2: {}", answer2);
}

fn process_part1(input: &str) -> Result<u32, std::io::Error> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut line = line.split_whitespace();
        left.push(line.next().unwrap().parse::<u32>().unwrap());
        right.push(line.next().unwrap().parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    let diff = zip(left, right).map(|x| x.0.abs_diff(x.1)).sum();

    Ok(diff)
}

fn process_part2(input: &str) -> Result<u32, std::io::Error> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let mut line = line.split_whitespace();
        left.push(line.next().unwrap().parse::<u32>().unwrap());
        let rval = line.next().unwrap().parse::<u32>().unwrap();
        {
            let count = right.entry(rval).or_insert(0);
            *count += 1;
        }
    }

    let diff = left.into_iter().map(|x| x * right.get(&x).unwrap_or(&0u32)).sum();

    Ok(diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = 
        "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

        assert_eq!(11, process_part1(input).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = 
        "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

        assert_eq!(31, process_part2(input).unwrap());
    }
}