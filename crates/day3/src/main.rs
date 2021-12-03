const INPUT: &str = include_str!("input.txt");

fn main() {
    // println!("input: {:?}", INPUT);
    println!("problem1: {}", solve_problem1(INPUT));
    println!("problem2: {}", solve_problem2(INPUT));
}

#[allow(unused)]
fn solve_problem1(input: &str) -> u32 {
    let ns = numbers(input);
    let (gamma, epsilon): (u32, u32) = (0..12).fold((0, 0), |(gamma, epsilon), power| {
        let place = 1 << power;
        let ones = ns.iter().filter(|n| *n & place > 0).count();
        if ones > ns.len() / 2 {
            (gamma + place, epsilon)
        } else {
            (gamma, epsilon + place)
        }
    });

    gamma * epsilon
}

#[allow(unused)]
fn solve_problem2(input: &str) -> u32 {
    let ns = numbers(input);
    let o2 = filter(ns.clone(), |ones, zeroes| ones >= zeroes);
    let co2 = filter(ns.clone(), |ones, zeroes| ones < zeroes);
    println!("o2: {:?} co2: {:?}", o2, co2);
    o2[0] * co2[0]
}

fn filter<F>(ns: Vec<u32>, keep_ones: F) -> Vec<u32>
where
    F: Fn(usize, usize) -> bool,
{
    (0..12).rev().fold(ns, |candidates, power| {
        if candidates.len() == 1 {
            candidates
        } else {
            let place = 1 << power;
            let ones = candidates.iter().filter(|&&n| n & place > 0).count();
            let zeroes = candidates.len() - ones;
            let keep_ones = keep_ones(ones, zeroes);

            candidates
                .into_iter()
                .filter(|&n| {
                    if keep_ones {
                        n & place > 0
                    } else {
                        n & place == 0
                    }
                })
                .collect()
        }
    })
}

fn numbers(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect::<Vec<_>>()
}
