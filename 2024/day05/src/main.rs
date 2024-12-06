use std::{collections::{HashMap, HashSet}, fs::{self}};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    // Calculate part 1 answer
    let answer1 = process_part1(input.as_str());
    println!("Answer part 1: {}", answer1);

    // Calculate part 2 answer
    let answer2 = process_part2(input.as_str());
    println!("Answer part 2: {}", answer2);
}

fn process_part1(input: &str) -> usize {
    check_order(input, false)
}

fn process_part2(input: &str) -> usize {
    check_order(input, true)
}

fn check_order(input: &str, fixups: bool) -> usize {
    let mut count = 0;
    let mut after_order_map: HashMap<usize, HashSet<usize>> = HashMap::new(); 
    for line in input.lines() {
        let line = line.trim();
        let orders: Vec<&str> = line.split("|").collect();
        if orders.len() > 1 {
            let orders: Vec<usize> = orders.into_iter().map(|n|n.parse::<usize>().unwrap()).collect();
            let x = orders[0];
            let y = orders[1];
            // Build set of pages that must follow this page
            let s = after_order_map.entry(x).or_insert(HashSet::new());
            s.insert(y);
        }
        let pages: Vec<&str> = line.split(",").collect();
        if pages.len() > 1 {
            let mut pages: Vec<usize> = pages.into_iter().map(|n|n.parse::<usize>().unwrap()).collect();
            let mut fixed = false;
            'check_order: loop {
                let mut visited: HashMap<usize, usize> = HashMap::new();
                for i in 0..pages.len() {
                    let page = pages[i];
                    visited.insert(page, i);
                    // Check is any visited pages preceded this page which shouldn't have
                    if let Some(after_set) = after_order_map.get(&page) {
                        for entry in after_set {
                            if let Some(v) = visited.get(entry) {
                                if fixups {
                                    pages.swap(i, *v);
                                    fixed = true;
                                    continue 'check_order; // Try again
                                }
                                break 'check_order;
                            }
                        }
                    }
                }
                if !fixups || (fixups && fixed) {
                    let mid = pages[pages.len() / 2];
                    count += mid;
                }
                break;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = 
            "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            ";

        assert_eq!(143, process_part1(input.into()));
    }

    #[test]
    fn test_part2() {
        let input =
            "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            ";

        
        assert_eq!(123, process_part2(input.into()));
    }
}