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
    multiply(input, true)
}

fn process_part2(input: &str) -> u32 {
    multiply(input, false)
}

#[derive(PartialEq)]
enum ScanState {
    Token,
    Params,
}

fn multiply(memory: &str, only_mul: bool) -> u32 {
    let mut state: ScanState = ScanState::Token;
    let mut total: u32 = 0;
    let mut param = 0u32;
    let mut param_total = 1u32;

    const MUL_TOKEN: &str = "mul(";
    const DO_TOKEN: &str = "do()";
    const DONT_TOKEN: &str = r"don't(";

    let mut last_token = 0;
    let mut enable_mul = true;

    for i in 0..memory.len() {
        if state == ScanState::Token {
            let token = &memory[last_token..i];
            if enable_mul && token.ends_with(MUL_TOKEN) {
                last_token = i;
                state = ScanState::Params;
                param = 0;
                param_total = 1;
            } else if !only_mul {
                if token.ends_with(DO_TOKEN) {
                    enable_mul = true;
                } else if token.ends_with(DONT_TOKEN) {
                   enable_mul = false;
                }
            }
        }
        if state == ScanState::Params {
            let ch = &memory[i..i+1].chars().last().unwrap();
            if ch.is_digit(10) {
                param = (param * 10) + ch.to_digit(10).unwrap();
            } else if *ch == ',' {
                param_total *= param;
                param = 0;
            } else if *ch == ')'  {
                param_total *= param;
                total += param_total;
                state = ScanState::Token;
            } else {
                // Invalid input, reset to token scan
                state = ScanState::Token;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(161, process_part1(input.into()));
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(48, process_part2(input.into()));
    }
}