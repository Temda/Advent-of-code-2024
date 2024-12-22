use std::fs;
use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let equations = parse(&input);

    let calibration_result_part1: f64 = equations
        .iter()
        .filter(|(v, nums)| check_part1(*v, nums.clone()))
        .map(|eq| eq.0)
        .sum();

    println!("solution part 1: {calibration_result_part1}");

    let calibration_result_part2: f64 = equations
        .iter()
        .filter(|(v, nums)| check_part2(*v, nums.clone()))
        .map(|eq| eq.0)
        .sum();

    println!("solution part 2: {calibration_result_part2}");
    Ok(())
}

fn parse(input: &String) -> Vec<(f64, Vec<f64>)> {
    input
        .lines()
        .filter_map(|line| line.split_once(':'))
        .map(|(n, m)| {
            (
                n.parse::<f64>().unwrap(),
                m.trim()
                    .split_whitespace()
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec()
}

fn check_part1(target: f64, nums: Vec<f64>) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    let n = nums.last().unwrap();
    return check_part1(target / n, nums.clone().drain(..(nums.len() - 1)).collect())
        || check_part1(target - n, nums.clone().drain(..(nums.len() - 1)).collect());
}

fn check_part2(target: f64, nums: Vec<f64>) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    let n = nums[0];
    let m = nums[1];

    let mut nm = vec![n * m];
    nm.extend(nums.clone().drain(2..));

    let mut na = vec![n + m];
    na.extend(nums.clone().drain(2..));

    let mut nc = vec![(n * 10f64.powi(m.log10() as i32 + 1) + m)];
    nc.extend(nums.clone().drain(2..));

    return check_part2(target, nm) || check_part2(target, na) || check_part2(target, nc);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "10: 2 5\n20: 4 5\n";
        let expected = vec![
            (10.0, vec![2.0, 5.0]),
            (20.0, vec![4.0, 5.0]),
        ];
        let result = parse(&input.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_check_part1() {
        assert!(check_part1(10.0, vec![2.0, 5.0]));
        assert!(!check_part1(10.0, vec![2.0, 3.0]));
    }

    #[test]
    fn test_check_part2() {
        assert!(check_part2(10.0, vec![2.0, 5.0]));
        assert!(check_part2(20.0, vec![4.0, 5.0]));
        assert!(!check_part2(10.0, vec![2.0, 3.0]));
    }
}