use advent_of_code_2025::read_from_file;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct DialState {
    position: i32,
    zero_passes: i32,
}

impl DialState {
    fn new(start_position: i32) -> Self {
        Self {
            position: start_position,
            zero_passes: 0,
        }
    }

    fn apply_rotation(&mut self, direction: Direction, value: i32) {
        match direction {
            Direction::Right => {
                self.position += value;
                self.zero_passes += self.position / 100;
                self.position %= 100;
            }
            Direction::Left => {
                if self.position == 0 {
                    // If starting at zero, only count complete revolutions
                    self.zero_passes += value / 100;
                } else if value >= self.position {
                    // If value exceeds position, get the number of complete revolutions relative to current position and add 1 for crossing zero
                    self.zero_passes += (value - self.position) / 100 + 1;
                }
                // Add a large number before modulo to avoid negative values
                self.position = (self.position - value + 10000) % 100;
            }
        }
    }

    fn is_at_zero(&self) -> bool {
        self.position == 0
    }
}

fn parse_instruction(input: &str) -> Option<(Direction, i32)> {
    let input = input.trim();
    let first_char = input.chars().next()?;
    
    let direction = match first_char.to_ascii_lowercase() {
        'l' => Direction::Left,
        'r' => Direction::Right,
        _ => return None,
    };
    
    let value = input[1..].parse::<i32>().ok()?;
    Some((direction, value))
}

fn parse_instructions(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                parse_instruction(line)
            }
        })
        .collect()
}

fn solve_part_one(input: &str) -> i32 {
    let instructions = parse_instructions(input);
    let mut state = DialState::new(50);
    let mut count = 0;
    
    for (direction, value) in instructions {
        state.apply_rotation(direction, value);

        // Only count if we are at zero after rotation
        if state.is_at_zero() {
            count += 1;
        }
    }
    
    count
}

fn solve_part_two(input: &str) -> i32 {
    let instructions = parse_instructions(input);
    let mut state = DialState::new(50);
    
    for (direction, value) in instructions {
        state.apply_rotation(direction, value);
    }
    
    state.zero_passes
}

fn main() {
    let input = read_from_file("inputs/day01.txt");
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
        let input = read_from_file("inputs/day01_test.txt");
        
        let expected = 3;
        let result = solve_part_one(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let input = read_from_file("inputs/day01_test.txt");
        
        let expected = 6;
        let result = solve_part_two(&input);
        assert_eq!(result, expected);
    }
}