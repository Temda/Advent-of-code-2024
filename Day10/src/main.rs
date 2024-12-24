use std::collections::VecDeque;
use std::fs;
use std::io::{self};

#[derive(Debug)]
struct TopographicMap {
    map: Vec<Vec<u32>>,
}

impl TopographicMap {
    fn from_input(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| {
                line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
            })
            .collect();
        TopographicMap { map }
    }

    fn find_trailheads(&self) -> Vec<(usize, usize)> {
        let mut trailheads = vec![];
        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                if value == 0 {
                    trailheads.push((row_idx, col_idx));
                }
            }
        }
        trailheads
    }

    fn score_trailhead(&self, start: (usize, usize)) -> usize {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
        let mut queue = VecDeque::new();
        let mut score = 0;

        queue.push_back(start);
        visited[start.0][start.1] = true;

        while let Some((row, col)) = queue.pop_front() {
            let current_height = self.map[row][col];
            if current_height == 9 {
                score += 1;
            }

            for &(dr, dc) in &directions {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;

                if new_row >= 0
                    && new_row < self.map.len() as isize
                    && new_col >= 0
                    && new_col < self.map[0].len() as isize
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if !visited[new_row][new_col]
                        && self.map[new_row][new_col] == current_height + 1
                    {
                        visited[new_row][new_col] = true;
                        queue.push_back((new_row, new_col));
                    }
                }
            }
        }

        score
    }

    fn rating_trailhead(&self, start: (usize, usize)) -> usize {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut queue = VecDeque::new();
        let mut rating = 0;
    
        queue.push_back(start);
    
        while let Some((row, col)) = queue.pop_front() {
            let current_height = self.map[row][col];
            if current_height == 9 {
                rating += 1;
                continue; 
            }
    
            for &(dr, dc) in &directions {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;
    
                if new_row >= 0
                    && new_row < self.map.len() as isize
                    && new_col >= 0
                    && new_col < self.map[0].len() as isize
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
    
                    if self.map[new_row][new_col] == current_height + 1 {
                        queue.push_back((new_row, new_col));
                    }
                }
            }
        }
    
        rating
    }

    fn total_score(&self) -> usize {
        let trailheads = self.find_trailheads();
        trailheads
            .iter()
            .map(|&trailhead| self.score_trailhead(trailhead))
            .sum()
    }

    fn total_rating(&self) -> usize {
        let trailheads = self.find_trailheads();
        trailheads
            .iter()
            .map(|&trailhead| self.rating_trailhead(trailhead))
            .sum()
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?.trim().to_string();
    let topo_map = TopographicMap::from_input(&input);

    let total_score = topo_map.total_score();
    println!("part 1: {}", total_score);

    let total_rating = topo_map.total_rating();
    println!("part 2: {}", total_rating);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_score() {
        let input = "0123\n1234\n8765\n9876";
        let topo_map = TopographicMap::from_input(input);
        assert_eq!(topo_map.total_score(), 1);
    }

    #[test]
    fn test_larger_example() {
        let input = "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        let topo_map = TopographicMap::from_input(input);
        assert_eq!(topo_map.total_score(), 36);
        assert_eq!(topo_map.total_rating(), 81);
    }
}