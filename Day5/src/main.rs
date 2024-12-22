use std::collections::HashMap;
use std::fs;
use std::io;

use itertools::Itertools;

type PageOrderingRules = HashMap<u32, Vec<u32>>;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("Input content:\n{}", input);
    
    let (input_rules, input_updates) = input.split_once("\n\n")
    .expect("Input is not correctly formatted. Expected a blank line to separate rules and updates.");

    let rules = parse_rules(input_rules);
    let updates = parse_updates(input_updates);

    let valid_updates: u32 = updates
        .iter()
        .filter(|pages| pages.is_sorted_by(|n, m| applies(&rules, m, n)))
        .map(|pages| pages[pages.len() / 2])
        .sum();

    println!("solution part 1: {valid_updates}");
    Ok(())
}

fn parse_rules(input: &str) -> PageOrderingRules {
    let mut rules = PageOrderingRules::new();
    for rule in input.lines() {
        let (n, m) = rule.split_once('|').unwrap();
        rules
            .entry(m.parse().unwrap())
            .or_default()
            .push(n.parse().unwrap());
    }
    rules
}

fn parse_updates(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn applies(rules: &PageOrderingRules, k: &u32, v: &u32) -> bool {
    rules.get(k).map(|rule| rule.contains(v)).unwrap_or(false)
}
