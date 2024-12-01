use std::collections::HashMap;

fn main() {
    let input = include_str!(".././input1.txt");
    let output_part1 = solution_part1(split_lines(input));
    let output_part2 = solution_part2(split_lines(input));

    dbg!(output_part1);
    dbg!(output_part2);
}

fn split_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solution_part1(input: Vec<&str>) -> u32 {
    let (first_list, second_list): (Vec<u32>, Vec<u32>) = input
        .into_iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    let mut first_list_sorted = first_list.clone();
    let mut second_list_sorted = second_list.clone();
    first_list_sorted.sort();
    second_list_sorted.sort();

    first_list_sorted
        .iter()
        .zip(second_list_sorted.iter())
        .map(|(a, b)| u32::abs_diff(*a, *b))
        .sum()
}

fn solution_part2(input: Vec<&str>) -> u32 {
    let (left_list, right_list): (Vec<u32>, Vec<u32>) = input
        .into_iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    let mut right_freq_map = HashMap::new();
    for num in &right_list {
        *right_freq_map.entry(num).or_insert(0) += 1;
    }

    left_list
        .iter()
        .map(|&num| {
            let count = *right_freq_map.get(&num).unwrap_or(&0);
            num * count
        })
        .sum()
}
