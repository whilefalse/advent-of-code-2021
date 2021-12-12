use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const STEPS: u32 = 100;
const WIDTH: i32 = 10;

fn main() {
    println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u32 {
    let mut grid = parse(input);
    let mut flash_count = 0;
    for _ in 0..STEPS {
        flash_count += step(&mut grid);
    }
    flash_count
}

#[allow(unused)]
fn solve_problem2(input: &str) -> u32 {
    let mut grid = parse(input);
    let mut step_n = 0;
    loop {
        let flash_count = step(&mut grid);
        if flash_count == (WIDTH * WIDTH) as u32 {
            return step_n + 1;
        }
        step_n += 1;
    }
}

fn step(grid: &mut Vec<u8>) -> u32 {
    let mut queue: Vec<usize> = vec![];
    let mut flashed: HashSet<usize> = HashSet::new();
    let mut flash_count = 0;

    // Initially process all squares
    for (i, _) in grid.iter().enumerate() {
        queue.push(i);
    }

    // Carry on until we've processed all squares
    while !queue.is_empty() {
        let i = queue.pop().unwrap();
        grid[i] += 1;
        if grid[i] > 9 && !flashed.contains(&i) {
            // Push neighbours & set flashed
            queue.extend(neighbours(i).iter());
            flashed.insert(i);
        }
    }

    // Reset all flashed to 0
    for &i in flashed.iter() {
        grid[i] = 0;
        flash_count += 1;
    }
    flash_count
}

fn neighbours(i: usize) -> Vec<usize> {
    let mut ns = vec![];
    let i = i as i32;

    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if i + dx >= 0
                && i + dx < WIDTH * WIDTH
                && i + dy * WIDTH >= 0
                && i + dy * WIDTH < WIDTH * WIDTH
                && (dx != 0 || dy != 0)
                && i / WIDTH == (i + dx) / WIDTH
            {
                ns.push((i + dx + dy * WIDTH) as usize);
            }
        }
    }
    ns
}

fn parse(input: &str) -> Vec<u8> {
    input
        .replace("\n", "")
        .chars()
        .map(|n| n.to_string().parse::<u8>().unwrap())
        .collect::<Vec<_>>()
}
