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

fn step(grid: &mut Vec<Vec<u8>>) -> u32 {
    let mut queue: Vec<(usize, usize)> = vec![];
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut flash_count = 0;

    // Initially process all squares
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            queue.push((x, y));
        }
    }

    // Carry on until we've processed all squares
    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        grid[y][x] += 1;
        if grid[y][x] > 9 && !flashed.contains(&(x, y)) {
            // Push neighbours & set flashed
            queue.extend(neighbours((x, y)).iter());
            flashed.insert((x, y));
        }
    }

    // Reset all flashed to 0
    for &(x, y) in flashed.iter() {
        grid[y][x] = 0;
        flash_count += 1;
    }
    flash_count
}

fn neighbours((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut ns = vec![];
    let x = x as i32;
    let y = y as i32;

    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if x + dx >= 0
                && x + dx < WIDTH
                && y + dy >= 0
                && y + dy < WIDTH
                && (dx != 0 || dy != 0)
            {
                ns.push(((x + dx) as usize, (y + dy) as usize));
            }
        }
    }
    ns
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
