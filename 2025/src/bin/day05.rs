use advent_of_code_2025::read_from_file;

fn get_product_id_ranges(input: &str) -> Vec<(u64, u64)> {
    // Parse lines until the first empty line
    let mut ranges = Vec::new();
    for line in input.trim().lines() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('-');

        let start: u64  = parts.next().unwrap().parse().unwrap();
        let end: u64 = parts.next().unwrap().parse().unwrap();
        
        ranges.push((start, end));
    }
    ranges
}

fn get_products_from_list(input: &str) -> Vec<u64> {
    // Parse lines after the first empty line
    let mut products = Vec::new();
    let mut lines = input.trim().lines();

    // Skip until the first empty line
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
    }

    // Now parse the product IDs
    for line in lines {
        let product_id: u64 = line.parse().unwrap();
        products.push(product_id);
    }

    products
}

fn solve_part_one(input: &str) -> u64 {
    let ranges = get_product_id_ranges(input);
    let products = get_products_from_list(input);

    // Determine how many product IDs are valid (fall within any of the ranges - inclusive)
    let mut valid_count = 0;
    'product_loop: for product in products {
        for (start, end) in &ranges {
            if product >= *start && product <= *end {
                valid_count += 1;

                // No need to check other ranges for this product, move to the next product
                continue 'product_loop;
            }
        }
    }

    valid_count
}

fn solve_part_two(input: &str) -> u64 {
    let ranges = get_product_id_ranges(input);
    
    // Take the difference between the max and min of all ranges to get the total number of valid product IDs
    // To ensure that we don't double count overlapping ranges, we will first merge the ranges
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();

    // First, sort ranges by start value
    let mut sorted_ranges: Vec<(u64, u64)> = ranges.clone();
    sorted_ranges.sort_by_key(|&(start, _)| start);

    for (start, end) in sorted_ranges {
        // Check if this range overlaps with the last range in merged_ranges
        if let Some((_, last_end)) = merged_ranges.last_mut() {

            if start <= *last_end {
                // Ranges overlap or are contiguous, merge them
                *last_end = (*last_end).max(end);
            } else {
                // No overlap, add the new range
                merged_ranges.push((start, end));
            }
            
        } else {
            // merged_ranges is empty, add the first range
            merged_ranges.push((start, end));
        }
    }

    // Now calculate the total number of valid product IDs from the merged ranges
    let mut total_valid = 0;
    for (start, end) in merged_ranges {
        total_valid += end - start + 1; // +1 because ranges are inclusive
    }
    
    total_valid
}

fn main() {
    let input = read_from_file("inputs/day05.txt");
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
        let input = read_from_file("inputs/day05_test.txt");
        
        let expected = 3;
        let result = solve_part_one(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let input = read_from_file("inputs/day05_test.txt");
        
        let expected = 14;
        let result = solve_part_two(&input);
        assert_eq!(result, expected);
    }
}