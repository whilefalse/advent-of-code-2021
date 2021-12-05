use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn main() {
    println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u32 {
    let horizontal_and_vertical_lines = parse(input)
        .into_iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect::<Vec<_>>();
    solve(horizontal_and_vertical_lines)
}

#[allow(unused)]
fn solve_problem2(input: &str) -> u32 {
    solve(parse(input))
}

fn solve(lines: Vec<Line>) -> u32 {
    duplicate_points(all_points_in_lines(lines))
}

fn all_points_in_lines(lines: Vec<Line>) -> Vec<Point> {
    lines
        .iter()
        .flat_map(|line| {
            let mut points = vec![];
            let mut x = line.start.x;
            let mut y = line.start.y;
            let dx = line.end.x.cmp(&line.start.x) as i32;
            let dy = line.end.y.cmp(&line.start.y) as i32;

            while x != line.end.x || y != line.end.y {
                points.push(Point { x: x, y: y });
                x += dx;
                y += dy;
            }
            points.push(Point { x: x, y: y });
            points
        })
        .collect::<Vec<_>>()
}

fn duplicate_points(points: Vec<Point>) -> u32 {
    points
        .into_iter()
        .map(|p| (p, ()))
        .into_group_map()
        .into_iter()
        .filter(|(_, v)| v.len() > 1)
        .count() as u32
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let x = line
                .split(" -> ")
                .map(|point| {
                    point
                        .split(",")
                        .map(|coord| coord.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Line {
                start: Point {
                    x: x[0][0],
                    y: x[0][1],
                },
                end: Point {
                    x: x[1][0],
                    y: x[1][1],
                },
            }
        })
        .collect::<Vec<_>>()
}
