use advent_of_code_2025::read_from_file;

fn get_max_battery_joltage(arr: &[u8]) -> Option<(usize, u8)> {
    let mut idx: usize = 0;
    let mut max: u8 = b'0';

    for (i, val) in arr.iter().copied().enumerate() {
        if val > max {
            max = val;
            idx = i;

            // Early exit if we find the maximum possible digit
            if val == b'9' {
                break;
            }
        }
    }

    Some((idx, max))
}

fn get_max_joltage(line: &str, max_batteries: usize) -> u64 {
    let mut battery_subset = line.as_bytes();
    let mut digits: Vec<u8> = vec![0; max_batteries];

    // Extract the maximum digits iteratively
    for i in 0..max_batteries {
        let (idx, byte) = get_max_battery_joltage(
            &battery_subset[..battery_subset.len() - ((max_batteries - 1) - i)]
        )
        .unwrap();

        digits[i] = byte;
        battery_subset = &battery_subset[idx + 1..];
    }

    unsafe { std::str::from_utf8_unchecked(&digits) }
        .parse::<u64>()
        .unwrap_or(0)
}

fn solve_part_one(input: &str) -> u64 {
    input.trim()
        .lines()
        .map(|line| {
            get_max_joltage(line, 2)
        })
        .sum()
}

fn solve_part_two(input: &str) -> u64 {
    input.trim()
        .lines()
        .map(|line| {
            get_max_joltage(line, 12)
        })
        .sum()
}

fn main() {
    let input = read_from_file("inputs/day03.txt");
    let part_one_result = solve_part_one(&input);
    println!("Part One Result: {}", part_one_result);

    let part_two_result = solve_part_two(&input);
    println!("Part Two Result: {}", part_two_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_from_file("inputs/day03_test.txt");
        
        let expected = 357;
        let result = solve_part_one(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let input = read_from_file("inputs/day03_test.txt");
        
        let expected = 3121910778619;
        let result = solve_part_two(&input);
        assert_eq!(result, expected);
    }
}