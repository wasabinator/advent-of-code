use std::{collections::{HashSet, VecDeque}, fs::{self}, ops::Neg};

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
    search(
        &build_grid(input),
        &vec![
            (-1, -1), (0, -1), (1, -1), 
            (-1, 0),            (1, 0),
            (-1, 1),  (0, 1),   (1, 1)
        ],
        1,
        'X',
        &vec!['M', 'A', 'S'],
        None,
    )
}

fn process_part2(input: &str) -> u32 {
    search(
        &build_grid(input),
        &vec![
            (-1, -1), (1, -1), 
            (-1, 1),  (1, 1)
        ],
        -1,
        'A',
        &vec!['M', 'A', 'S'],
        Some(2),
    )
}

fn build_grid(input: &str) -> Vec<Vec<char>> {
    let rows: Vec<&str> = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    rows.into_iter().map(|s|s.chars().collect()).collect()
}

fn search(
    data: &Vec<Vec<char>>,
    scan_offsets: &Vec<(isize, isize)>,
    direction: isize,
    anchor: char,
    term: &Vec<char>,
    require_count: Option<u32>,
) -> u32 {
    let mut count: u32 = 0;
    let width = data[0].len();
    let height = data.len();

    for x in 0..width {
        for y in 0..height {
            let ch = data[y][x];
            if ch == anchor {
                let matches = scan(&data, &term, &scan_offsets, direction, x, y);
                match require_count {
                    Some(n) => {
                        if matches == n {
                            count += 1
                        }
                    }
                    None => {
                        count += matches;
                    }
                }
            }
        }
    }

    count
}

fn scan(
    data: &Vec<Vec<char>>,
    term: &Vec<char>,
    scan_offsets: &Vec<(isize, isize)>,
    direction: isize,
    x: usize,
    y: usize,
) -> u32 {
    let mut count: u32 = 0;
    let width = data[0].len();
    let height = data.len();
    let term_max_idx = term.len() - 1;

    let mut queue: VecDeque<(usize, usize, usize, (isize, isize))> = VecDeque::new();

    for offset in scan_offsets {
        let direction = (offset.0 * direction, offset.1 * direction);
        let x = x.checked_add_signed(offset.0);
        let y = y.checked_add_signed(offset.1);
        if x.is_some() && y.is_some() {
            queue.push_back((0, x.unwrap(), y.unwrap(), direction));
        }
    }

    while !queue.is_empty() {
        let (idx, x, y, direction) = queue.pop_front().unwrap();
        let next_char = term[idx];
        if x < width && y < height && data[y][x] == next_char {
            if idx < term_max_idx {
                let x = x.checked_add_signed(direction.0); 
                let y = y.checked_add_signed(direction.1);
                if x.is_some() && y.is_some() {
                    let x = x.unwrap();
                    let y = y.unwrap();
                    queue.push_back((idx+1, x, y, direction));
                }
            } else {
                count += 1;
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
            "MMMSXXMASM
             MSAMXMSMSA
             AMXSXMAAMM
             MSAMASMSMX
             XMASAMXAMM
             XXAMMXXAMA
             SMSMSASXSS
             SAXAMASAAA
             MAMMMXMMMM
             MXMXAXMASX";

        assert_eq!(18, process_part1(input.into()));
    }

    #[test]
    fn test_part2() {
        let input = 
            "MMMSXXMASM
             MSAMXMSMSA
             AMXSXMAAMM
             MSAMASMSMX
             XMASAMXAMM
             XXAMMXXAMA
             SMSMSASXSS
             SAXAMASAAA
             MAMMMXMMMM
             MXMXAXMASX";

        assert_eq!(9, process_part2(input.into()));
    }
}