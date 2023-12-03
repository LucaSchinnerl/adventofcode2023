use regex::Regex;
use std::collections::HashMap;

#[aoc::main(03)]
fn main(input: &str) -> (u32, u32) {
    solution(input)
}

fn solution(input: &str) -> (u32, u32) {
    let ranges = get_digit_ranges(input);
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let total_points = ranges.iter().fold(HashMap::new(), |mut acc, r| {
        add_adjacent(&map, r, &mut acc);
        acc
    });

    let p1 = total_points.values().map(|v| v.iter().sum::<u32>()).sum();
    let p2 = total_points
        .values()
        .filter(|v| v.len() > 1)
        .map(|v| v.iter().product::<u32>())
        .sum();

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
    let re = Regex::new(r"\d+").unwrap();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            re.find_iter(line).map(move |mat| {
                let x_start = mat.start().saturating_sub(1);
                let x_end = if mat.end() != line.len() {
                    mat.end()
                } else {
                    mat.end() - 1
                };
                let y_start = y.saturating_sub(1);
                let y_end = if y != line.len() - 1 { y + 1 } else { y };

                NumberRange {
                    x_start,
                    x_end,
                    y_start,
                    y_end,
                    number: mat.as_str().parse::<u32>().unwrap(),
                }
            })
        })
        .collect()
}

fn add_adjacent(map: &[Vec<char>], r: &NumberRange, total_points: &mut HashMap<Point, Vec<u32>>) {
    (r.y_start..=r.y_end).for_each(|y| {
        (r.x_start..=r.x_end).for_each(|x| {
            let temp_point = Point { x, y };
            if !map[y][x].is_numeric() && map[y][x] != '.' {
                total_points
                    .entry(temp_point)
                    .or_insert(Vec::new())
                    .push(r.number);
            }
        });
    });
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
