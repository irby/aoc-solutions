use advent_of_code_2025::read_from_file;

fn parse_ranges(input: &str) -> Result<Vec<Vec<u64>>, String> {
    let mut result = Vec::new();
    
    for range_str in input.split(',') {
        let range_str = range_str.trim();
        
        if let Some((start_str, end_str)) = range_str.split_once('-') {
            let start: u64 = start_str.trim()
                .parse()
                .map_err(|_| format!("Invalid number: {}", start_str))?;
            let end: u64 = end_str.trim()
                .parse()
                .map_err(|_| format!("Invalid number: {}", end_str))?;
            
            let range: Vec<u64> = (start..=end).collect();
            result.push(range);
        } else {
            return Err(format!("Invalid range format: {}", range_str));
        }
    }
    
    Ok(result)
}

fn has_repeated_digits(num: u64) -> bool {
    let s = num.to_string();
    if s.len() < 2 || s.len() % 2 != 0 {
        return false;
    }

    let pattern_length = s.len() / 2;

    let pattern = &s[..pattern_length];
    return s.chars()
        .collect::<Vec<_>>()
        .chunks(pattern_length)
        .all(|chunk| chunk.iter().collect::<String>() == pattern);
}

fn has_repeated_digits_v2(num: u64) -> bool {
    let s = num.to_string();
    if s.len() < 2 {
        return false;
    }

    // Check if the string is made up of a repeated pattern
    for pattern_len in 1..=s.len()/2 {
        if s.len() % pattern_len == 0 {
            let pattern = &s[..pattern_len];
            if s.chars().collect::<Vec<_>>()
                .chunks(pattern_len)
                .all(|chunk| chunk.iter().collect::<String>() == pattern) {
                return true;
            }
        }
    }
    
    false
}


fn solve_part_one(input: &str) -> i64 {
    let id_ranges = parse_ranges(input).unwrap();
    let mut result = 0;

    id_ranges.iter().for_each(|range| {
        let repeated_nums: Vec<u64> = range.iter()
            .filter(|&&num| has_repeated_digits(num))
            .copied()
            .collect();
        
        if !repeated_nums.is_empty() {
            result += repeated_nums.iter().map(|&x| x as i64).sum::<i64>();
            // println!("Range contains repeated digits: {:?}", repeated_nums);
        }
    });
    
    result
}

fn solve_part_two(input: &str) -> i64 {
    let id_ranges = parse_ranges(input).unwrap();
    let mut result = 0;

    id_ranges.iter().for_each(|range| {
        let repeated_nums: Vec<u64> = range.iter()
            .filter(|&&num| has_repeated_digits_v2(num))
            .copied()
            .collect();
        
        if !repeated_nums.is_empty() {
            result += repeated_nums.iter().map(|&x| x as i64).sum::<i64>(); 
            // println!("Range contains repeated digits: {:?}", repeated_nums);
        }
    });
    
    result
}


fn main() {
    let input = read_from_file("inputs/day02.txt");
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
        let input = read_from_file("inputs/day02_test.txt");
        
        let expected = 1227775554;
        let result = solve_part_one(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let input = read_from_file("inputs/day02_test.txt");
        
        let expected = 4174379265;
        let result = solve_part_two(&input);
        assert_eq!(result, expected);
    }
}