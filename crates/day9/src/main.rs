const INPUT: &str = include_str!("input.txt");

fn main() {
    // println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> i32 {
    let (grid, width) = parse(input);
    let width = width as i32;

    low_points(&grid, width).iter().map(|(_, x)| x + 1).sum()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    let (grid, width) = parse(input);
    let width = width as i32;
    let low_points = low_points(&grid, width);

    let mut sizes = low_points
        .into_iter()
        .map(|(i, val)| basin_size(&grid, width, i))
        .collect::<Vec<_>>();
    println!("{:?}", sizes);
    sizes.sort();
    sizes.iter().rev().take(3).fold(1, |acc, item| acc * item)
}

fn basin_size(grid: &Vec<i32>, width: i32, index: usize) -> i32 {
    println!("basin size for index {}", index);
    let mut basin: Vec<usize> = vec![];
    let mut queue: Vec<usize> = vec![];
    queue.push(index);
    while !queue.is_empty() {
        let i = queue.pop().unwrap();
        if basin.contains(&i) {
            continue;
        }
        println!("popped: {}", i);
        basin.push(i as usize);
        let ns = neighbours(grid.len() as i32, width, i as i32);
        println!("neighbours: {:?}", ns);
        for i in ns {
            if grid[i] != 9 {
                println!("pushing index {}, val {}", i, grid[i]);
                queue.push(i)
            }
        }
    }
    println!("result: {:?} {}", basin, basin.len());
    basin.len() as i32
}

fn low_points(grid: &Vec<i32>, width: i32) -> Vec<(usize, i32)> {
    grid.into_iter()
        .enumerate()
        .filter(|(i, val)| {
            let i = *i as i32;
            let ns = neighbours(grid.len() as i32, width, i);
            ns.into_iter().all(|n| **val < grid[n])
        })
        .map(|(i, x)| (i, *x))
        .collect::<Vec<_>>()
}

fn neighbours(size: i32, width: i32, index: i32) -> Vec<usize> {
    let mut ns: Vec<usize> = vec![];
    if index % width > 0 && index > 0 {
        ns.push((index - 1) as usize);
    }
    if index % width != (width - 1) && index < size - 1 {
        ns.push((index + 1) as usize);
    }
    if index - width >= 0 {
        ns.push((index - width) as usize);
    }
    if index + width < size {
        ns.push((index + width) as usize);
    }
    ns
}

fn parse(input: &str) -> (Vec<i32>, usize) {
    let width = input.lines().next().unwrap().len();
    let grid = input
        .replace("\n", "")
        .chars()
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    (grid, width)
}
