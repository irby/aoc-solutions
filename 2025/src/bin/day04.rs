use advent_of_code_2025::read_from_file;

type Grid = Vec<Vec<char>>;

fn parse_grid(input: &str) -> Grid {
    input.trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn count_adjacent_matching(grid: &Grid, row: usize, col: usize, target: char) -> usize {
    // This array represents the 8 possible directions around a cell
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];
    
    directions.iter()
        .filter(|(x_mutation, y_mutation)| {
            let new_row_index = row as i32 + x_mutation;
            let new_col_index = col as i32 + y_mutation;
            
            if new_row_index >= 0 && new_col_index >= 0 {
                // Find the character at the new position, if it exists and determine if it matches the target character
                grid.get(new_row_index as usize)
                    .and_then(|new_row: &Vec<char>| new_row.get(new_col_index as usize))
                    .map_or(false, |&c| c == target)
            } else {
                false
            }
        })
        .count()
}

fn get_removable_rolls(grid: &Grid, roll_char: char, max_roll_count: usize) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(row, chars)| {
            chars.iter()
                .enumerate()
                .filter_map(move |(col, &ch)| {
                    if ch == roll_char && count_adjacent_matching(grid, row, col, roll_char) < max_roll_count {
                        Some((row, col))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

fn solve_part_one(input: &str) -> u64 {
    let grid = parse_grid(input);
    get_removable_rolls(&grid, '@', 4).len() as u64
}

fn solve_part_two(input: &str) -> u64 {
    let mut grid = parse_grid(input);
    let mut total_removed = 0;

    loop {
        let removed_rolls = get_removable_rolls(&grid, '@', 4);
        
        if removed_rolls.is_empty() {
            break;
        }

        // Mark rolls that were removed in the current iteration
        for (row, col) in &removed_rolls {
            grid[*row][*col] = 'X';
        }

        total_removed += removed_rolls.len() as u64;
    }

    total_removed
}

fn main() {
    let input = read_from_file("inputs/day04.txt");
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
        let input = read_from_file("inputs/day04_test.txt");
        
        let expected = 13;
        let result = solve_part_one(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let input = read_from_file("inputs/day04_test.txt");
        
        let expected = 43;
        let result = solve_part_two(&input);
        assert_eq!(result, expected);
    }
}