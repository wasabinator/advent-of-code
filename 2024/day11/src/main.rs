use std::{collections::{HashMap, HashSet, VecDeque}, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    let s = input.as_str();

    // Calculate part 1 answer
    // let count = count_stones(s, 25);
    // println!("Answer part 1: {}", count);

    let count = count_stones(s, 75);
    println!("Answer part 2: {}", count);
}

fn count_stones(input: &str, blinks: usize) -> usize {
    let stones: Vec<usize> = input.split_whitespace().map(|s|s.parse::<usize>().unwrap()).collect();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut score = 0;
    for stone in stones {
        println!("Stone {}:", stone);
        score += calc_score(stone, 0, blinks, &mut cache);
    }
    score
}

fn calc_score(stone: usize, blink: usize, blinks: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    //println!("Calc Score: {}, {} of {}", stone, blink, blinks);

    if blink >= blinks {
        return 1;
    }

    if let Some(cached) = cache.get(&(stone, blink)) {
         return *cached;
    }

    let mut result = 0;
    if stone == 0 {
        result = calc_score(1, blink + 1, blinks, cache);
    } else if count_digits(stone) & 1 == 0 {
        let len = count_digits(stone);
        let (l, r) = split_digits(stone, len);
        result = calc_score(l, blink + 1, blinks, cache) + calc_score(r, blink + 1, blinks, cache)
    } else {
        result = calc_score(stone * 2024, blink + 1, blinks, cache);
    }

    cache.insert((stone, blink), result);
    
    return result;
}

fn count_digits(num: usize) -> usize {
    let mut n = num;
    if n == 0 {
        return 1;
    }
    let mut digits = 0;
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    digits
}

fn split_digits(num: usize, len: usize) -> (usize, usize) {
    let mut l = num;
    let mut r = 0;
    let mut f = 1;
    for _ in 0..len/2 {
        r += (l % 10) * f;
        l /= 10;
        f *= 10;
    }
    (l, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(1, count_digits(0));
        assert_eq!(2, count_digits(10));
        assert_eq!(2, count_digits(15));
        assert_eq!(3, count_digits(100));
        assert_eq!(3, count_digits(111));
    }

    #[test]
    fn test_split_digits() {
        assert_eq!((10, 12), split_digits(1012, 4));
        assert_eq!((101, 245), split_digits(101245, 6));
    }

    #[test]
    fn test_part1() {
        let input = 
            "125 17";

        let total = count_stones(input.into(), 6);
        assert_eq!(22, total);
    }
}