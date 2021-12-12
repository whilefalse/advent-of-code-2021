use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let now = Instant::now();
    println!("Part 1: {}", solve_problem1(INPUT));
    println!("Part 2 {}", solve_problem2(INPUT));
    println!("Time {:?}", now.elapsed());
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u32 {
    let parsed = parse(input);
    let paths = find_all_paths(&parsed, &mut vec!["start"], true);
    paths
}

#[allow(unused)]
fn solve_problem2(input: &str) -> u32 {
    let parsed = parse(input);
    let paths = find_all_paths(&parsed, &mut vec!["start"], false);
    paths
}

fn find_all_paths<'a>(
    hash: &HashMap<&'a str, Vec<&'a str>>,
    current: &mut Vec<&'a str>,
    retrod: bool,
) -> u32 {
    let current_head = current[current.len() - 1];
    if current_head == "end" {
        return 1;
    }
    hash[current_head]
        .iter()
        .map(|conn| {
            let retreading = is_small_cave(conn) && current.contains(conn);
            if (retreading && retrod) || *conn == "start" {
                0
            } else {
                current.push(conn);
                let found = find_all_paths(hash, current, retrod || retreading);
                current.pop();
                found
            }
        })
        .sum()
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

fn parse<'a>(input: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut hash = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once("-").unwrap();
        hash.entry(from).or_insert(vec![]).push(to);
        hash.entry(to).or_insert(vec![]).push(from);
    }
    hash
}
