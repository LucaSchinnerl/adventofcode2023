use std::collections::HashSet;

#[aoc::main(09)]
fn main(input: &str) -> (i64, i64) {
    (solution(input, "part1"), solution(input, "part2"))
}

fn solution(input: &str, part: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            predict(&numbers, part)
        })
        .sum()
}

fn predict(numbers: &Vec<i64>, part: &str) -> i64 {
    if HashSet::<&i64>::from_iter(numbers.iter()).is_empty() {
        return 0;
    }
    match part {
        "part1" => predict(&calc_diff(numbers), part) + numbers[numbers.len() - 1],
        "part2" => numbers[0] - predict(&calc_diff(numbers), part),
        _ => panic!("Invalid part"),
    }
}

fn calc_diff(numbers: &Vec<i64>) -> Vec<i64> {
    let mut diff = Vec::new();
    for i in 1..numbers.len() {
        diff.push(numbers[i] - numbers[i - 1]);
    }
    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(solution(input, "part1"), 114);
        assert_eq!(solution(input, "part2"), 2);
    }
}
