#[aoc::main(04)]
fn main(input: &str) -> (u32, u32) {
    solution(input)
}

fn solution(input: &str) -> (u32, u32) {
    let mut stack: Vec<u32> = (0..input.chars().filter(|&c| c == '\n').count() + 1)
        .map(|_| 1)
        .collect();
    let p1 = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (cards, numbers) = line.split_once(" | ").unwrap();
            let winning = cards
                .split(": ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let pow = numbers
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .fold(0, |acc, x| if winning.contains(&x) { acc + 1 } else { acc });
            (i + 1..=(i + pow)).for_each(|idx| {
                stack[idx] += stack[i];
            });
            2_u32.pow(pow as u32 - 1)
        })
        .sum();

    let p2 = stack.iter().sum();

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solution(input), (13, 30));
    }
}
