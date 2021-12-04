const INPUT: &str = include_str!("input.txt");
const BOARD_SIZE: usize = 5;

fn main() {
    println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let (numbers, boards) = parse(input);
    let mut called = [false; 100];
    for number in numbers {
        called[number] = true;
        if let Some(w) = boards.iter().find(|&board| is_winner(board, called)) {
            let sum: usize = w.iter().filter(|&&x| !called[x]).sum();
            return sum * number;
        }
    }
    0
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let (numbers, boards) = parse(input);
    let mut called = [false; 100];
    let mut remaining = boards.iter().collect::<Vec<_>>();
    for number in numbers {
        called[number] = true;
        let before = remaining.clone();
        remaining.retain(|board| !is_winner(board, called));

        if remaining.is_empty() {
            let sum: usize = before[0].iter().filter(|&&x| !called[x]).sum();
            return sum * number;
        }
    }
    0
}

fn is_winner(board: &Vec<usize>, called: [bool; 100]) -> bool {
    (0..BOARD_SIZE).any(|i| {
        // Row is complete?
        (0..BOARD_SIZE).all(|j| called[board[(i * BOARD_SIZE + j)]]) ||
        // Column is complete?
        (0..BOARD_SIZE).all(|j| called[board[(j * BOARD_SIZE + i)]])
    })
}

fn parse(input: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut split = input.split("\n\n");
    let called = split
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let boards = split
        .map(|board| {
            board
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (called, boards)
}
