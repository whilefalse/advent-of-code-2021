use std::collections::HashSet;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Fold {
    Horizontal(i32),
    Vertical(i32),
}

fn main() {
    let now = Instant::now();
    println!("Part 1: {}", solve_problem1(INPUT));
    println!("Part 2: {}", solve_problem2(INPUT));
    println!("Time {:?}", now.elapsed());
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let (points, folds) = parse(input);
    let points = fold(&points, &folds[0]);
    points.len()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    let (points, folds) = parse(input);
    let folded = folds.into_iter().fold(points, |acc, f| fold(&acc, &f));
    print_grid(&folded);
    0
}

fn fold(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    points
        .into_iter()
        .filter_map(|&Point { x, y }| {
            let new_point = match *fold {
                Fold::Horizontal(fold) => {
                    if x > fold {
                        Point {
                            x: fold * 2 - x,
                            y: y,
                        }
                    } else {
                        Point { x: x, y: y }
                    }
                }
                Fold::Vertical(fold) => {
                    if y > fold {
                        Point {
                            x: x,
                            y: fold * 2 - y,
                        }
                    } else {
                        Point { x: x, y: y }
                    }
                }
            };
            if new_point.x >= 0 && new_point.y >= 0 {
                Some(new_point)
            } else {
                None
            }
        })
        .collect()
}

fn print_grid(points: &HashSet<Point>) {
    let maxx = points.iter().map(|Point { x, y }| x).max().unwrap();
    let maxy = points.iter().map(|Point { x, y }| y).max().unwrap();

    for y in 0..=*maxy {
        for x in 0..=*maxx {
            if points.contains(&Point { x, y }) {
                print!("||");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

#[allow(unused)]
fn parse(input: &str) -> (HashSet<Point>, Vec<Fold>) {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points = points
        .lines()
        .map(|point| point.split_once(",").unwrap())
        .map(|(x, y)| Point {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap(),
        })
        .collect();

    let folds = folds
        .lines()
        .map(|f| {
            let instruction = f.replace("fold along ", "");
            let (direction, position) = instruction.split_once("=").unwrap();
            let position = position.parse::<i32>().unwrap();
            match direction {
                "x" => Fold::Horizontal(position),
                "y" => Fold::Vertical(position),
                _ => panic!(),
            }
        })
        .collect();

    (points, folds)
}
