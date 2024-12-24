use std::{collections::HashMap, fs, io};

fn split_number(num: usize) -> (usize, usize) {
    let num_str = num.to_string();
    let mid = num_str.len() / 2;
    let left = num_str[..mid].parse::<usize>().unwrap_or(0);
    let right = num_str[mid..].parse::<usize>().unwrap_or(0);
    (left, right)
}

fn evolve_stones(stones: &[usize], blinks: usize, cache: &mut HashMap<(usize, usize), Vec<usize>>) -> Vec<usize> {
    let mut stones = stones.to_vec();
    for _ in 0..blinks {
        let mut new_stones = Vec::new();
        for &stone in &stones {
            if let Some(cached_result) = cache.get(&(stone, blinks)) {
                new_stones.extend(cached_result.clone());
            } else {
                let result = if stone == 0 {
                    vec![1]
                } else if stone.to_string().len() % 2 == 0 {
                    let (left, right) = split_number(stone);
                    vec![left, right]
                } else {
                    vec![stone * 2024]
                };
                cache.insert((stone, blinks), result.clone());
                new_stones.extend(result);
            }
        }
        stones = new_stones;
    }
    stones
}

fn read_input(file_path: &str) -> io::Result<Vec<usize>> {
    let input = fs::read_to_string(file_path)?;
    Ok(input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect())
}

fn part_one(input: &[usize]) {
    let blinks = 25;
    let mut cache = HashMap::new();
    let final_stones = evolve_stones(input, blinks, &mut cache);
    println!("Part 1: Number of stones after {} blinks: {}", blinks, final_stones.len());
}

fn part_two(input: &[usize]) {
    let blinks = 75;
    let mut cache = HashMap::new();
    let final_stones = evolve_stones(input, blinks, &mut cache);
    println!("Part 2: Number of stones after {} blinks: {}", blinks, final_stones.len());
}

fn main() -> io::Result<()> {
    let input = read_input("input.txt")?;
    part_one(&input);
    part_two(&input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_number() {
        assert_eq!(split_number(1234), (12, 34));
        assert_eq!(split_number(5678), (56, 78));
        assert_eq!(split_number(0), (0, 0));
    }

    #[test]
    fn test_evolve_stones() {
        let stones = vec![1234, 5678, 0];
        let mut cache = HashMap::new();
        let result = evolve_stones(&stones, 1, &mut cache);
        assert_eq!(result, vec![12, 34, 56, 78, 1]);
    }

    #[test]
    fn test_read_input() {
        let input = "1234 5678 0";
        let expected = vec![1234, 5678, 0];
        let result = read_input_from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let input = vec![1234, 5678, 0];
        part_one(&input);
    }

    #[test]
    fn test_part_two() {
        let input = vec![1234, 5678, 0];
        part_two(&input);
    }

    fn read_input_from_str(input: &str) -> io::Result<Vec<usize>> {
        Ok(input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect())
    }
}