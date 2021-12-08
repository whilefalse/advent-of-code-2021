use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Problem {
    inputs: Vec<String>,
    outputs: Vec<String>,
}
fn main() {
    println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> usize {
    let parsed = parse(input);
    parsed
        .iter()
        .map(|problem| {
            let mut numbers = [""; 10];
            numbers[8] = with_unique_length(&problem.inputs, 7);
            numbers[1] = with_unique_length(&problem.inputs, 2);
            numbers[4] = with_unique_length(&problem.inputs, 4);
            numbers[7] = with_unique_length(&problem.inputs, 3);
            problem
                .outputs
                .iter()
                .filter(|output| {
                    output == &numbers[8]
                        || output == &numbers[1]
                        || output == &numbers[4]
                        || output == &numbers[7]
                })
                .count()
        })
        .sum()
}

#[allow(unused)]
fn solve_problem2(input: &str) -> usize {
    let parsed = parse(input);

    // 1, 4, 7, 8 - ALREADY HAVE
    // A 4 + TWO OTHERS MUST BE A 9
    // 5 part numbers: 2, 3, 5
    // A 5 part number which shares only 2 parts from a 4 must be a 2
    // A 3 is a 5 part number with only 1 differene from a 2
    // A 5 is the left over 5 part number
    // A 6 is a 5 with one extra bit
    // A 0 is the left over
    parsed
        .iter()
        .map(|problem| {
            let mut numbers = [""; 10];
            numbers[8] = with_unique_length(&problem.inputs, 7);
            numbers[1] = with_unique_length(&problem.inputs, 2);
            numbers[4] = with_unique_length(&problem.inputs, 4);
            numbers[7] = with_unique_length(&problem.inputs, 3);
            numbers[9] = similar_to(&problem.inputs, 6, numbers[4], 4);
            numbers[2] = similar_to(&problem.inputs, 5, numbers[4], 2);
            numbers[3] = similar_to(&problem.inputs, 5, numbers[2], 4);
            numbers[5] = problem
                .inputs
                .iter()
                .find(|input| input.len() == 5 && input != &numbers[2] && input != &numbers[3])
                .unwrap();
            numbers[6] = similar_to_but_not(&problem.inputs, 6, numbers[5], 5, numbers[9]);
            numbers[0] = problem
                .inputs
                .iter()
                .find(|input| input.len() == 6 && input != &numbers[6] && input != &numbers[9])
                .unwrap();

            let mut iter = problem.outputs.iter();
            let one = iter.next().unwrap();
            let two = iter.next().unwrap();
            let three = iter.next().unwrap();
            let four = iter.next().unwrap();

            find(numbers, one) * 1000
                + find(numbers, two) * 100
                + find(numbers, three) * 10
                + find(numbers, four)
        })
        .sum()
}

fn with_unique_length(inputs: &Vec<String>, length: usize) -> &str {
    inputs.iter().find(|input| input.len() == length).unwrap()
}

fn similar_to<'a>(
    inputs: &'a Vec<String>,
    length: usize,
    similar_to: &'a str,
    shared_length: usize,
) -> &'a str {
    inputs
        .iter()
        .find(|input| input.len() == length && shared_chars(input, similar_to) == shared_length)
        .unwrap()
}

fn similar_to_but_not<'a>(
    inputs: &'a Vec<String>,
    length: usize,
    similar_to: &'a str,
    shared_length: usize,
    not: &str,
) -> &'a str {
    inputs
        .iter()
        .find(|input| {
            input.len() == length
                && shared_chars(input, similar_to) == shared_length
                && input != &not
        })
        .unwrap()
}

fn find(numbers: [&str; 10], string: &str) -> usize {
    numbers.iter().position(|x| x == &string).unwrap()
}

fn shared_chars(one: &str, two: &str) -> usize {
    let mut shared = 0;
    for ch in one.chars() {
        if two.contains(ch) {
            shared += 1;
        }
    }
    shared
}

fn parse(input: &str) -> Vec<Problem> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" | ");
            let inputs = split.next().unwrap();
            let inputs_parsed = inputs
                .split(" ")
                .map(|x| x.chars().sorted().collect::<String>())
                .collect::<Vec<String>>();
            let outputs = split.next().unwrap();
            let outputs_parsed = outputs
                .split(" ")
                .map(|x| x.chars().sorted().collect::<String>())
                .collect::<Vec<String>>();
            Problem {
                inputs: inputs_parsed,
                outputs: outputs_parsed,
            }
        })
        .collect::<Vec<_>>()
}
