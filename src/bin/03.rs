use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/03.in");

    let r = part1(input);

    println!("Result: {}", r);
}

fn part1(input: &str) -> u32 {
    let ranges = get_digit_ranges(input);

    let map = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut res = 0;
    for r in ranges {
        if check_adjacent(&map, &r){
            res += r.number as u32;
        }
    }
    res
}

#[derive(Debug)]
struct NumberRange {
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
    number: u32
}

fn get_digit_ranges(input: &str) -> Vec<NumberRange> {
    let mut result = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for (y, line) in input.lines().enumerate() {
        for mat in re.find_iter(line) {
            let mut x_start = mat.start();
            if x_start != 0{
                x_start -= 1;
            }
            let mut x_end = mat.end() - 1;
            if x_end != line.len() - 1 {
                x_end += 1;
            }

            let mut y_start = y;
            let mut y_end = y;

            if y_start != 0 {
                y_start -= 1;
            }


            if y_end != line.len() - 1 {
                y_end += 1;
            }

            result.push(
                NumberRange { x_start: x_start, x_end: x_end, y_start: y_start, y_end: y_end, number: mat.as_str().parse::<u32>().unwrap() }
            );

        }
    }
    result
}

fn check_adjacent(map: &Vec<Vec<char>>, r: &NumberRange) -> bool {

    for y in r.y_start..=r.y_end {
        for x in r.x_start..=r.x_end {
            if !map[y][x].is_numeric() && map[y][x] != '.'  {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!(part1(input), 4361);
    }
}