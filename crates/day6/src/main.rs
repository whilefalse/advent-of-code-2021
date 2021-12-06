const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    solve(input, 80)
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    solve(input, 256)
}

fn solve(input: &str, generations: u32) -> usize {
    let mut fish = parse(input);
    for _ in 0..generations {
        let zeros = fish[0];
        for i in 0..8 {
            fish[i] = fish[i + 1];
        }
        fish[8] = zeros;
        fish[6] += zeros;
    }
    fish.iter().sum()
}

fn parse(input: &str) -> [usize; 9] {
    let mut fish = [0; 9];
    for number in input.split(",") {
        let parsed = number.parse::<usize>().unwrap();
        fish[parsed] += 1;
    }
    fish
}
