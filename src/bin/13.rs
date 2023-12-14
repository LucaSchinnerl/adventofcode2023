#[aoc::main(13)]
fn main(input: &str) -> (usize, usize) {
    let p1 = solution(input, &part1_comparison);
    let p2 = solution(input, &part2_comparison);
    (p1, p2)
}

fn solution(input: &str, comparison: &dyn Fn(Vec<Vec<char>>, Vec<Vec<char>>) -> bool) -> usize {
    let parsed_maps: Vec<Vec<Vec<char>>> = input.split("\n\n").map(parse_single_map).collect();

    let mut res = 0;
    for sketch in parsed_maps {
        res += (find_morror(&sketch, comparison) * 100)
            .max(find_morror(&transpose(sketch), comparison));
    }
    res
}

fn parse_single_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn find_morror(
    sketch: &Vec<Vec<char>>,
    comparison: &dyn Fn(Vec<Vec<char>>, Vec<Vec<char>>) -> bool,
) -> usize {
    for distance_to_top in 1..sketch.len() {
        let distance_to_bottom = sketch.len() - distance_to_top;

        let diff = distance_to_bottom.min(distance_to_top);

        let top = sketch[distance_to_top - diff..distance_to_top].to_vec();
        let mut bottom = sketch[distance_to_top..distance_to_top + diff].to_vec();

        bottom.reverse();

        if comparison(top, bottom) {
            return distance_to_top;
        }
    }
    0
}

fn part1_comparison(top: Vec<Vec<char>>, bottom: Vec<Vec<char>>) -> bool {
    top == bottom
}

fn part2_comparison(top: Vec<Vec<char>>, bottom: Vec<Vec<char>>) -> bool {
    top.iter()
        .zip(bottom.iter())
        .map(|(row_top, row_bottom)| {
            row_top
                .iter()
                .zip(row_bottom.iter())
                .filter(|(a, b)| a != b)
                .count()
        })
        .sum::<usize>()
        == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(solution(input, &part1_comparison), 405);
    }
}
