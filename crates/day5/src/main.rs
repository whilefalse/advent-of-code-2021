use std::collections::HashMap;

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
    // println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u32 {
    let lines = parse(input)
        .into_iter()
        // Use only horizonal and vertical lines
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect::<Vec<Line>>();
    // println!("lines {:#?}", lines);
    let points = all_points_in_lines(&lines);
    let duplicate_points = duplicate_points(&points);
    // println!("points: {:?}", points);
    // println!("duplicate points: {:?}", duplicate_points);
    duplicate_points
}

#[allow(unused)]
fn solve_problem2(input: &str) -> i32 {
    todo!()
}

fn all_points_in_lines(lines: &Vec<Line>) -> Vec<Point> {
    lines
        .iter()
        .flat_map(|line| {
            // println!("line: {:?}", line);
            let mut points = vec![];
            // Horizontal
            if line.start.y == line.end.y {
                let mut x = line.start.x;
                while x <= line.end.x {
                    points.push(Point {
                        x: x,
                        y: line.start.y,
                    });
                    x += 1
                }
            }
            // Vertical
            else {
                let mut y = line.start.y;
                while y <= line.end.y {
                    points.push(Point {
                        x: line.start.x,
                        y: y,
                    });
                    y += 1
                }
            }
            // println!("this points: {:#?}", points);
            points
        })
        .collect::<Vec<_>>()
}

fn duplicate_points(points: &Vec<Point>) -> u32 {
    let mut map: HashMap<&Point, u32> = HashMap::new();
    for point in points {
        if map.contains_key(point) {
            map.insert(point, map.get(point).unwrap() + 1);
        } else {
            map.insert(point, 1);
        }
    }
    map.into_iter()
        .filter(|(_, v): &(&Point, u32)| *v > 1)
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
                    x: std::cmp::min(x[0][0], x[1][0]),
                    y: std::cmp::min(x[0][1], x[1][1]),
                },
                end: Point {
                    x: std::cmp::max(x[0][0], x[1][0]),
                    y: std::cmp::max(x[0][1], x[1][1]),
                },
            }
        })
        .collect::<Vec<_>>()
}
