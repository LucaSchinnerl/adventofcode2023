use regex::Regex;
use std::collections::HashMap;

#[aoc::main(03)]
fn main(input: &str) -> (u32, u32) {
    let r = solution(input);

    (r.0, r.1)
}

fn solution(input: &str) -> (u32, u32) {
    let ranges = get_digit_ranges(input);

    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut total_points = HashMap::new();

    ranges.iter().for_each(|r| {
        add_adjecient(&map, r, &mut total_points);
    });

    let mut p1 = 0;
    let mut p2 = 0;

    total_points.iter().for_each(|(_, v)| {
        p1 += v.iter().sum::<u32>();
        if v.len() > 1 {
            p2 += v.iter().product::<u32>();
        }
    });

    (p1, p2)
}

#[derive(Debug)]
struct NumberRange {
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
    number: u32,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Point {
    x: usize,
    y: usize,
}

fn get_digit_ranges(input: &str) -> Vec<NumberRange> {
    let mut result = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for (y, line) in input.lines().enumerate() {
        for mat in re.find_iter(line) {
            let mut x_start = mat.start();
            x_start = x_start.saturating_sub(1);
            let mut x_end = mat.end() - 1;
            if x_end != line.len() - 1 {
                x_end += 1;
            }

            let mut y_start = y;
            let mut y_end = y;

            y_start = y_start.saturating_sub(1);

            if y_end != line.len() - 1 {
                y_end += 1;
            }

            result.push(NumberRange {
                x_start,
                x_end,
                y_start,
                y_end,
                number: mat.as_str().parse::<u32>().unwrap(),
            });
        }
    }
    result
}

fn add_adjecient(
    map: &Vec<Vec<char>>,
    r: &NumberRange,
    total_points: &mut HashMap<Point, Vec<u32>>,
) {
    for y in r.y_start..=r.y_end {
        for x in r.x_start..=r.x_end {
            if !map[y][x].is_numeric() && map[y][x] != '.' {
                let temp_point = Point { x, y };
                total_points
                    .entry(temp_point)
                    .or_insert(Vec::new())
                    .push(r.number);
                return ;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(solution(input), (4361, 467835));
    }
}
