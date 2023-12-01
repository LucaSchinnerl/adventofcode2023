fn main() {
    let input = include_str!("../input.txt");
    let part_one = part_one(input);
    let part_two = part_two(input);

    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<String>())
        .map(|s| {
            let first = s.chars().next().unwrap();
            let last = s.chars().last().unwrap();
            format!("{}{}", first, last)
        })
        .map(|s| s.parse::<u32>().unwrap())
        .sum::<u32>()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            replace_spelled_numbers(line)
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
        })
        .map(|s| {
            let first = s.chars().next().unwrap();
            let last = s.chars().last().unwrap();
            format!("{}{}", first, last)
        })
        .map(|s| s.parse::<u32>().unwrap())
        .sum::<u32>()
}

fn replace_spelled_numbers(input: &str) -> String {
    let spelled_numbers = [
        ("zero", "ze0o"),
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let mut result = input.to_string();
    for (spelled, actual) in &spelled_numbers {
        result = result.replace(spelled, actual);
    }
    result
}
