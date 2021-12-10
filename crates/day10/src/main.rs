const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    let lines = parse(input);
    lines.into_iter().map(|line| syntax_error_score(line)).sum()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> u64 {
    let lines = parse(input);
    let mut scores = lines
        .into_iter()
        .map(|line| completion_score(line))
        .filter(|score| score > &0)
        .collect::<Vec<_>>();
    scores.sort();
    scores[(scores.len() - 1) / 2]
}

fn syntax_error_score(line: &str) -> i32 {
    let mut stack: Vec<char> = vec![];
    for ch in line.chars() {
        if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
            stack.push(ch);
        } else {
            let start = stack.pop().unwrap();
            let expected_end = expected_end(start);
            if ch != expected_end {
                return match ch {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!(),
                };
            }
        }
    }
    0
}

fn completion_score(line: &str) -> u64 {
    let mut stack: Vec<char> = vec![];
    for ch in line.chars() {
        if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
            stack.push(ch);
        } else {
            let start = stack.pop().unwrap();
            let expected_end = expected_end(start);
            if ch != expected_end {
                return 0;
            }
        }
    }
    if !stack.is_empty() {
        let mut score: u64 = 0;
        while !stack.is_empty() {
            let start = stack.pop().unwrap();
            let expected_end = expected_end(start);
            score = score * 5
                + (match expected_end {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!(),
                });
        }
        score
    } else {
        0
    }
}

fn expected_end(start: char) -> char {
    match start {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<_>>()
}
