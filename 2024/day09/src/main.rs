use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");

    let s = input.as_str();

    // Calculate part 1 answer
    let checksum = calc_defragged_sectors_checksum(s);
    println!("Answer part 1: {}", checksum);

    // Calculate part 2 answer
    let checksum = calc_defragged_files_checksum(s);
    println!("Answer part 2: {}", checksum);
}

fn calc_defragged_sectors_checksum(input: &str) -> usize {
    let line: Vec<usize> = input.trim().chars().map(|n| n.to_digit(10).unwrap() as usize).collect();
    let mut disk_image: &mut Vec<isize> = &mut Vec::new();

    let mut file_id = 0;
    for i in 0..line.len() {
        let n = line[i];
        let v: isize;
        if i & 1 == 0 {
            v = file_id as isize;
            file_id += 1;
        } else {
            v = -1;
        }
        for _ in 0..n {
            disk_image.push(v);
        }
    }

    let len = disk_image.len();

    let mut first_free = 0;
    for i in 0..len {
        if disk_image[i] < 0 {
            first_free = i;
            break;
        }
    }

    'defrag: for i in (0..len).rev() {
        let v = disk_image[i];
        if v >= 0 {
            disk_image[first_free] = v;
            disk_image[i] = -1;
            loop {
                first_free += 1;
                if first_free >= i {
                    break 'defrag; // We're done
                }
                if disk_image[first_free] < 0 { // Found next free block, exit the scan
                    break;
                }
            }
        }
    }

    // Calc checksum
    let mut checksum = 0;
    for i in 0..len {
        let n = disk_image[i];
        if n < 0 {
            break;
        }
        checksum += i * n as usize;
    }

    checksum
}

fn calc_defragged_files_checksum(input: &str) -> usize {
    let line: Vec<usize> = input.trim().chars().map(|n| n.to_digit(10).unwrap() as usize).collect();
    let mut disk_image: &mut Vec<isize> = &mut Vec::new();
    let mut files: VecDeque<(usize, usize, usize)> = VecDeque::new();

    let mut file_id = 0;
    for i in 0..line.len() {
        let n = line[i];
        let v: isize;
        if i & 1 == 0 {
            v = file_id as isize;
            if n > 0 {
                files.push_back((file_id, disk_image.len(), n));
            }
            file_id += 1;
        } else {
            v = -1;
        }
        for _ in 0..n {
            disk_image.push(v);
        }
    }

    let len = disk_image.len();

    while !files.is_empty() {
        let next_file = files.pop_back().unwrap();
        let mut block_start = 0;
        let mut block_size = 0;
        for i in 0..next_file.1 {
            if disk_image[i] < 0 {
                if block_size == 0 {
                    block_start = i;
                }
                block_size += 1;
                if block_size == next_file.2 {
                    for n in 0..next_file.2 {
                        disk_image[block_start + n] = next_file.0 as isize;
                        disk_image[next_file.1 + n] = -1;
                    }
                    break;
                }
            } else {
                block_size = 0;
            }
        }
    }

    // Calc checksum
    let mut checksum = 0;
    for i in 0..len {
        let n = disk_image[i];
        if n >= 0 {
            checksum += i * n as usize;
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = 
            "2333133121414131402";

        let total = calc_defragged_sectors_checksum(input.into());
        assert_eq!(1928, total);
    }

    #[test]
    fn test_part2() {
        let input = 
            "2333133121414131402";

        let total: usize = calc_defragged_files_checksum(input.into());
        assert_eq!(2858, total);
    }
}