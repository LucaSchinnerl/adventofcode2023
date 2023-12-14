use std::collections::HashMap;

#[aoc::main(14)]
fn main(input: &str) -> (usize, usize) {
    // Collect input to vec
    let mut char_map = input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let p1 = calculate_weight(&gravity(char_map.clone()));

    let cycle_length = get_cycle_length(char_map.clone());
    let start_cycles = cycle_length.0;
    let cycle_length = cycle_length.1 - cycle_length.0;
    let total_cycles_needed = (1000000000 - start_cycles) % cycle_length;

    for _ in 0..(total_cycles_needed + start_cycles) {
        // print_map(&char_map);
        char_map = simulate_cycle(char_map);
    }
    let p2 = calculate_weight(&char_map);
    (p1, p2)
}

fn get_cycle_length(char_map: Vec<Vec<char>>) -> (usize, usize) {
    let mut hm = HashMap::new();
    let mut char_map = char_map;
    for i in 0..1000000 {
        char_map = simulate_cycle(char_map);
        if hm.contains_key(&char_map) {
            return (*hm.get(&char_map).unwrap(), i);
        }
        hm.insert(char_map.clone(), i);
    }
    panic!("No cycle found");
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

fn calculate_weight(char_map: &[Vec<char>]) -> usize {
    let mut weight = 0;
    char_map.iter().rev().enumerate().for_each(|(y, row)| {
        row.iter().for_each(|c| {
            if *c == 'O' {
                weight += y + 1;
            }
        });
    });
    weight
}

fn simulate_cycle(char_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated_map = char_map.clone();
    for _ in 0..4 {
        let new_char_map = gravity(rotated_map);
        rotated_map = rotate_map(new_char_map);
    }
    rotated_map
}

fn gravity(char_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(
        transpose(char_map.clone())
            .iter()
            .map(|row| {
                let mut map_idx = 0;
                let mut new_row = row.clone();
                row.iter().enumerate().for_each(|(x, c)| match c {
                    'O' => {
                        new_row[x] = '.';
                        new_row[map_idx] = 'O';
                        map_idx += 1;
                    }
                    '#' => {
                        map_idx = x + 1;
                    }
                    _ => {}
                });
                new_row
            })
            .collect::<Vec<Vec<char>>>(),
    )
}

fn rotate_map(char_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // To rotate a map, we transpose it and reverse each row
    transpose(char_map)
        .iter()
        .map(|row| row.iter().copied().rev().collect())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let char_map = input
            .lines()
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        assert_eq!(calculate_weight(&gravity(char_map)), 136);
    }
}
