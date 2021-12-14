use itertools::Itertools;
use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let now = Instant::now();
    println!("input: {:?}", INPUT);
    println!("Part 1: {}", solve_problem1(INPUT));
    println!("Part 2: {}", solve_problem2(INPUT));
    println!("Time {:?}", now.elapsed());
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u64 {
    run(input, 10)
}

#[allow(unused)]
fn solve_problem2(input: &str) -> u64 {
    run(input, 40)
}

fn run(input: &str, n: i32) -> u64 {
    let (template, mappings) = parse(input);
    let template = template.chars().collect::<Vec<_>>();
    let mut counts: HashMap<(char, char), u64> = HashMap::new();
    for window in template.windows(2) {
        let entry = counts.entry((window[0], window[1])).or_insert(0);
        *entry += 1;
    }
    for i in 0..n {
        println!("Step: {}", i);
        println!("counts: {:?}", counts);
        let mut new_counts = HashMap::new();
        for (&(a, b), count) in counts.iter() {
            let mapped = mappings.get(&(a, b)).unwrap();
            let one = (a, *mapped);
            let two = (*mapped, b);
            let entry_1 = new_counts.entry(one).or_insert(0);
            *entry_1 += count;
            let entry_2 = new_counts.entry(two).or_insert(0);
            *entry_2 += count;
        }
        counts = new_counts;
    }

    let mut letters: HashMap<char, u64> = HashMap::new();
    letters.insert(template[0], 1);
    letters.insert(template[template.len() - 1], 1);
    for ((a, b), count) in counts {
        let entry1 = letters.entry(a).or_insert(0);
        *entry1 += count;
        let entry2 = letters.entry(b).or_insert(0);
        *entry2 += count;
    }
    println!("Letters: {:?}", letters);
    let letters = letters.iter().map(|(_, c)| c).collect::<Vec<_>>();
    (**letters.iter().max().unwrap() - **letters.iter().min().unwrap()) / 2
}

#[allow(unused)]
fn parse(input: &str) -> (String, HashMap<(char, char), char>) {
    let mut lines = input.lines();
    let template = lines.next().unwrap();
    lines.next();
    let mappings = lines
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            let f_chars = from.chars().collect::<Vec<_>>();
            let t_chars = to.chars().collect::<Vec<_>>();

            ((f_chars[0], f_chars[1]), t_chars[0])
        })
        .collect();
    (template.to_owned(), mappings)
}
