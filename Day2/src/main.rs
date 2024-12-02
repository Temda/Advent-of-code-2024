fn main() {
    
    let input = include_str!("../input.txt");

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();

    let safe_count_part1 = reports
        .iter()
        .filter(|&report| is_safe(report))
        .count();
    
    let safe_count_part2 = reports
        .iter()
        .filter(|&report| is_safe(report) || can_be_made_safe(report))
        .count();

    println!("Safe reports (Part 1): {}", safe_count_part1);
    println!("Safe reports (Part 2): {}", safe_count_part2);
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 0..report.len() - 1 {
        let diff = (report[i] - report[i + 1]).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

        if report[i] < report[i + 1] {
            is_decreasing = false;
        }
        if report[i] > report[i + 1] {
            is_increasing = false;
        }
    }

    is_increasing || is_decreasing
}

fn can_be_made_safe(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);

        if is_safe(&new_report) {
            return true;
        }
    }
    false
}
