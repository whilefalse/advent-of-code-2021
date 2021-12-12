use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u32 {
    let parsed = parse(input);
    let paths = find_all_paths(&parsed, &mut vec!["start"], false, false);
    paths
}

#[allow(unused)]
fn solve_problem2(input: &str) -> u32 {
    let now = Instant::now();
    let parsed = parse(input);
    let paths = find_all_paths(&parsed, &mut vec!["start"], true, false);
    println!("took {:?}", now.elapsed());
    paths
}

fn find_all_paths<'a>(
    hash: &HashMap<&'a str, Vec<&'a str>>,
    current: &mut Vec<&'a str>,
    can_retread_small_cave: bool,
    has_retrod_small_cave: bool,
) -> u32 {
    let current_head = current[current.len() - 1];
    if current_head == "end" {
        return 1;
    }
    hash[current_head]
        .iter()
        .map(|conn| {
            let retreading = is_small_cave(conn) && current.contains(conn);
            if (retreading && (has_retrod_small_cave || !can_retread_small_cave))
                || *conn == "start"
            {
                0
            } else {
                current.push(conn);
                let found = find_all_paths(
                    hash,
                    current,
                    can_retread_small_cave,
                    has_retrod_small_cave || retreading,
                );
                current.pop();
                found
            }
        })
        .sum()
}

fn is_small_cave(cave: &str) -> bool {
    cave.to_ascii_lowercase() == cave
}

fn parse<'a>(input: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut hash = HashMap::new();
    for line in input.lines() {
        let mut split = line.split("-");
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        hash.entry(from).or_insert(vec![]).push(to);
        hash.entry(to).or_insert(vec![]).push(from);
    }
    hash
}
