const INPUT: &str = include_str!("input.txt");

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn main() {
    println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    // println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    let (x, y) = commands(input)
        .iter()
        .fold((0, 0), |(x, y), elem| match elem {
            Command::Forward(n) => (x + n, y),
            Command::Down(n) => (x, y + n),
            Command::Up(n) => (x, y - n),
        });
    x * y
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    todo!()
}

fn commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|x| {
            let split: Vec<_> = x.split(" ").collect();
            let amount: i32 = split[1].parse().unwrap();
            match split[0] {
                "forward" => Command::Forward(amount),
                "down" => Command::Down(amount),
                "up" => Command::Up(amount),
                _ => panic!(),
            }
        })
        .collect::<Vec<_>>()
}
