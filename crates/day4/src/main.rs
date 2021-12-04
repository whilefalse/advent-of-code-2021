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
    let mut winner: Option<&Vec<usize>> = None;
    let mut last_called = 0;
    for number in numbers {
        last_called = number;
        called[number] = true;
        if let Some(w) = boards.iter().find(|&board| is_winner(board, called)) {
            winner = Some(w);
            break;
        }
    }
    match winner {
        None => panic!(),
        Some(w) => {
            let sum: usize = w.iter().filter(|&&x| !called[x]).sum();
            sum * last_called
        }
    }
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let (numbers, boards) = parse(input);
    let mut called = [false; 100];
    let mut last_called = 0;
    let mut winners: Vec<&Vec<usize>> = vec![];
    let mut remaining = boards.iter().collect::<Vec<_>>();

    for number in numbers {
        last_called = number;
        called[number] = true;

        winners = remaining
            .iter()
            .map(|b| *b)
            .filter(|board| is_winner(board, called))
            .collect::<Vec<_>>();
        remaining.retain(|board| !is_winner(board, called));

        if remaining.is_empty() {
            break;
        }
    }

    let w = &winners[0];
    let sum: usize = w.iter().filter(|&&x| !called[x]).sum();
    sum * last_called
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
