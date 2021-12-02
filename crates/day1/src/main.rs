const INPUT: &str = include_str!("input.txt");

fn main() {
    // println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    let ns = numbers(input);
    ns.iter()
        .zip(ns.iter().skip(1))
        .filter(|(one, two)| two > one)
        .count() as i32
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    // Two of the numbers are always the same in each 3-item window, so we only need to compare
    // the start of the first window to the end of the second window.
    let ns = numbers(input);
    ns.iter()
        .zip(ns.iter().skip(3))
        .filter(|(one, two)| two > one)
        .count() as i32
}

fn numbers(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}
