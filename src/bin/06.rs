use itertools::enumerate;
use regex::Regex;
use std::iter::zip;

#[aoc::main(06)]
fn main(input: &str) -> (u64, u64) {
    let (p1_input, p2_input) = parse_input(input);
    let p1 = solution(&p1_input);
    let p2 = solution(&p2_input);

    (p1, p2)
}

fn solution(races: &[Race]) -> u64 {
    races
        .iter()
        .map(|r| {
            (0..r.time).fold(0, |acc, t| {
                let dist = (r.time - t) * t;
                if r.record_distance < dist {
                    return acc + 1;
                }
                acc
            })
        })
        .product()
}

fn parse_input(input: &str) -> (Vec<Race>, Vec<Race>) {
    let (_time, _distance) = input.split_once('\n').unwrap();

    let re = Regex::new(r"\b\d+\b").unwrap();
    let numbers = re.captures_iter(input);
    let mut time = Vec::new();
    let mut distance = Vec::new();

    for (i, n) in enumerate(numbers) {
        if i < re.find_iter(input).count() / 2 {
            time.push(n[0].parse::<u64>().unwrap());
        } else {
            distance.push(n[0].parse::<u64>().unwrap());
        }
    }

    let p1_input = zip(time.clone(), distance.clone())
        .map(|(t, d)| Race {
            time: t,
            record_distance: d,
        })
        .collect();

    let p2_input = vec![Race {
        time: combine_numbers(time),
        record_distance: combine_numbers(distance),
    }];

    (p1_input, p2_input)
}

fn combine_numbers(numbers: Vec<u64>) -> u64 {
    let combined_str = numbers
        .into_iter()
        .map(|n| n.to_string())
        .collect::<String>();
    combined_str.parse::<u64>().unwrap()
}

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let (p1, p2) = parse_input(input);
        assert_eq!(solution(&p1), 288);
        assert_eq!(solution(&p2), 71503);
    }
}
