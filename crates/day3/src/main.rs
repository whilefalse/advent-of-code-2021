const INPUT: &str = include_str!("input.txt");

fn main() {
    // println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    // println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u32 {
    let ns = numbers(input);
    let (gamma, epsilon): (u32, u32) = (0..12).fold((0, 0), |(gamma, epsilon), power| {
        let place = 1 << power;
        let ones = ns.iter().filter(|n| *n & place > 0).count();
        if ones > ns.len() / 2 {
            (gamma + place, epsilon)
        } else {
            (gamma, epsilon + place)
        }
    });

    gamma * epsilon
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    todo!()
}

fn numbers(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect::<Vec<_>>()
}
