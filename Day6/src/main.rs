use std::fs;
use std::io;
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let input = read_input("input.txt")?;

    let output_part1 = part_1(&input);
    println!("Part 1: {}", output_part1);

    let output_part2 = part_2(&input);
    println!("Part 2: {}", output_part2);

    Ok(())
}

fn read_input(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let input = fs::read_to_string(file_path)?;
    Ok(input.lines().map(|l| l.chars().collect_vec()).collect())
}

fn part_1(map: &[Vec<char>]) -> usize {
    simulate(map).visited_positions.len()
}

fn part_2(map: &[Vec<char>]) -> usize {
    let mut obstructed_map = map.to_vec();
    (0..map.len())
        .flat_map(|y| (0..map[0].len()).map(move |x| (y, x)))
        .filter(|&(y, x)| {
            if map[y][x] != '.' {
                return false;
            }

            obstructed_map[y][x] = '#';
            let result = simulate(&obstructed_map).is_looping;
            obstructed_map[y][x] = '.';
            result
        })
        .count()
}


struct SimulationResult {
    visited_positions: HashSet<(usize, usize)>,
    is_looping: bool,
}

fn simulate(map: &[Vec<char>]) -> SimulationResult {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction = 0;
    let (mut y, mut x) = find_guard(map);

    let mut visited_positions = HashSet::new();
    let mut seen_states = HashSet::with_capacity(map.len() * map[0].len());

    loop {
        let state = (y, x, direction);
        if seen_states.contains(&state) {
            return SimulationResult {
                visited_positions,
                is_looping: true,
            };
        }
        seen_states.insert(state);

        visited_positions.insert((y, x));

        let (dy, dx) = directions[direction];
        let new_y = y as isize + dy;
        let new_x = x as isize + dx;

        match char_at(map, new_y, new_x) {
            ' ' | '#' => {
                direction = (direction + 1) % 4;
            }
            _ => {
                y = new_y as usize;
                x = new_x as usize;
            }
        }

        if new_y < 0 || new_x < 0 || new_y >= map.len() as isize || new_x >= map[0].len() as isize {
            break;
        }
    }

    SimulationResult {
        visited_positions,
        is_looping: false,
        
    }
}

fn find_guard(map: &[Vec<char>]) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '^') {
            return (y, x);
        }
    }
    panic!("Guard not found");
}

fn char_at(map: &[Vec<char>], y: isize, x: isize) -> char {
    if y < 0 || x < 0 || y as usize >= map.len() || x as usize >= map[0].len() {
        return ' ';
    }
    map[y as usize][x as usize]
}
