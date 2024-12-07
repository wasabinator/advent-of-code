use std::fs;

type Num = u128;
type Val = (Num, bool);
type Op = fn(Val, Num) -> Num;

fn add_op(a: Val, b: Num) -> Num {
    a.0 + b
}

fn mul_op(a: Val, b: Num) -> Num {
    a.0 * b
}

fn concat_op(a: Val, b: Num) -> Num {
    let mut arg = b;
    let mut acc: Num = a.0;

    while arg > 0 {
        arg /= 10;
        acc *= 10;
    }

    acc + b
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    let s = input.as_str();

    // Calculate part 1 answer
    let total = calc_valid_total(s, &vec![add_op, mul_op]);
    println!("Answer part 1: {}", total);

    // Calculate part 1 answer
    let total = calc_valid_total(s, &vec![add_op, mul_op, concat_op]);
    println!("Answer part 2: {}", total);
}

fn calc_valid_total(input: &str, operators: &Vec<Op>) -> Num {
    let lines: Vec<&str> = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    let mut total: Num = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(":").map(|s|s.trim()).collect();
        let expected = parts[0].parse::<Num>().unwrap();
        let operands: Vec<Num> = parts[1].split_whitespace().map(|s|s.trim().parse::<Num>().unwrap()).collect();

        let accumulator = operands[0];
        let calculated = calc(expected, 1, (accumulator, false), &operands, operators);
        if calculated.1 {
            //println!("YES {}={}", expected, calculated.0);
            total += calculated.0;
        } else {
            //println!("NO {}!={}", expected, calculated.0);
        }
    }

    total
}


fn calc(expected: Num, idx: usize, accumulator: Val, values: &Vec<Num>, operators: &Vec<Op>) -> Val {
    if accumulator.0 > expected || accumulator.1 || idx == values.len() {
        return accumulator;
    }

    let value = values[idx];
    for op in operators {
        let acc = calc(expected, idx+1, (op(accumulator, value), false), values, operators);
        if acc.1 {
            return acc;
        } else if acc.0 == expected && idx == values.len()-1 {
            println!("Found, i = {}, len = {}", idx, values.len());
            return (acc.0, true);
        }
    }

    accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = 
            "190: 10 19
             3267: 81 40 27
             83: 17 5
             156: 15 6
             7290: 6 8 6 15
             161011: 16 10 13
             192: 17 8 14
             21037: 9 7 18 13
             292: 11 6 16 20";

        let total= calc_valid_total(input.into(), &vec![add_op, mul_op]);
        assert_eq!(3749, total);
    }

    #[test]
    fn test_part2() {
        let input = 
            "190: 10 19
             3267: 81 40 27
             83: 17 5
             156: 15 6
             7290: 6 8 6 15
             161011: 16 10 13
             192: 17 8 14
             21037: 9 7 18 13
             292: 11 6 16 20";

        let total= calc_valid_total(input.into(), &vec![add_op, mul_op, concat_op]);
        assert_eq!(11387, total);
    }
}