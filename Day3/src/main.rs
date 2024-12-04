// Day 3: Mull It Over

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    let result_part1 = part1(input);
    println!("Total sum of multiplications (Part 1): {}", result_part1);

    let result_part2 = part2(input);
    println!("Total sum of multiplications (Part 2): {}", result_part2);
}

fn part1(input: &str) -> i32 {
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total_sum = 0;

    for caps in mul_pattern.captures_iter(input) {
        let x: i32 = caps[1].parse().unwrap();
        let y: i32 = caps[2].parse().unwrap();
        total_sum += x * y;
    }

    total_sum
}

fn part2(input: &str) -> i32 {
   let mut sum = 0;
   let mut enabled = true;
   let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

   for capture in regex.captures_iter(input) {
        if let Some(command) = capture.get(0) {
            match command.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {

                    if enabled {
                        let a: i32 = capture.get(1).unwrap().as_str().parse().expect("Invalid number");
                        let b: i32 = capture.get(2).unwrap().as_str().parse().expect("Invalid number");
                        sum += a * b;
                    }
                }
            }
        }
    }

    sum

}
