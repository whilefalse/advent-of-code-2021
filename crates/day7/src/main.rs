const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    let numbers = parse(input);
    let max = numbers.iter().max().unwrap();
    (0..=*max)
        .map(|n| numbers.iter().map(|num| (num - n).abs()).sum())
        .min()
        .unwrap()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    let numbers = parse(input);
    let max = numbers.iter().max().unwrap();
    (0..=*max)
        .map(|n| {
            numbers
                .iter()
                .map(|num| {
                    let moves = (num - n).abs();
                    // Sum of integers from 1 to `moves`
                    moves * (moves + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
fn parse(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}
