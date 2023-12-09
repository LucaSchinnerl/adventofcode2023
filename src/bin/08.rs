use num_integer::lcm;
use std::collections::HashMap;

#[aoc::main(08)]
fn main(input: &str) -> (u64, u64) {
    let instructions = input.lines().take(1).collect::<Vec<&str>>()[0];
    let (hash_map, start_positions) = parse_equations(input);
    (
        solution(&hash_map, vec!["AAA"], instructions),
        solution(&hash_map, start_positions, instructions),
    )
}

fn solution(
    hash_map: &HashMap<&str, (&str, &str)>,
    start_positions: Vec<&str>,
    instructions: &str,
) -> u64 {
    start_positions
        .iter()
        .map(|p| iterations_to_terminaton(hash_map, p, instructions))
        .fold(1, lcm)
}

fn iterations_to_terminaton(
    hash_map: &HashMap<&str, (&str, &str)>,
    start_position: &str,
    instructions: &str,
) -> u64 {
    let mut steps = 0;
    let mut position = start_position;
    let mut cycle = instructions.chars().cycle();
    loop {
        let instruction = cycle.next().unwrap();
        position = match instruction {
            'L' => hash_map.get(position).unwrap().0,
            'R' => hash_map.get(position).unwrap().1,
            _ => panic!("Invalid instruction"),
        };
        steps += 1;
        if position.ends_with('Z') {
            return steps;
        }
    }
}

fn parse_equations(input: &str) -> (HashMap<&str, (&str, &str)>, Vec<&str>) {
    let mut hash_map = HashMap::new();
    let mut start_positions = Vec::new();
    input.lines().skip(2).for_each(|l| {
        let parts: Vec<&str> = l.split(" = ").collect();
        let key = parts[0].trim();
        if key.ends_with('A') {
            start_positions.push(key);
        }
        let values = parts[1].trim_start_matches('(').trim_end_matches(')');
        let tuple_values: Vec<&str> = values.split(", ").collect();
        hash_map.insert(key, (tuple_values[0], tuple_values[1]));
    });
    (hash_map, start_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_par1() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let instructions = input.lines().take(1).collect::<Vec<&str>>()[0];
        let (hash_map, _start_positions) = parse_equations(input);
        assert_eq!(solution(&hash_map, vec!["AAA"], instructions), 6);
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let instructions = input.lines().take(1).collect::<Vec<&str>>()[0];
        let (hash_map, start_positions) = parse_equations(input);
        assert_eq!(solution(&hash_map, start_positions, instructions), 6);
    }
}
