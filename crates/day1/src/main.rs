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
    let ns = numbers(input);
    let sums = ns
        .iter()
        .zip(ns.iter().skip(1))
        .zip(ns.iter().skip(2))
        .map(|((one, two), three)| one + two + three)
        .collect::<Vec<_>>();
    sums.iter()
        .zip(sums.iter().skip(1))
        .filter(|(one, two)| two > one)
        .count() as i32
}

fn numbers(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}
