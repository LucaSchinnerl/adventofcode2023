use itertools::Itertools;
use std::usize;

#[aoc::main(07)]
fn main(input: &str) -> (u64, u64) {
    println!("{:?}", solution(input));
    println!("{:?}", solution(input));

    (0, 0)
}

fn solution(input: &str) -> u32 {
    let mut sorted_hands = input.lines().map(Hand::new).collect::<Vec<Hand>>();

    sorted_hands.sort_by(complex_criterion);
    sorted_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + h.bet * (i as u32 + 1))
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bet: u32,
}

impl Hand {
    fn new(line: &str) -> Self {
        let (cards, bet) = line.split_once(' ').unwrap();
        let parsed_bet = bet.parse::<u32>().unwrap();
        Self {
            cards: cards.to_string(),
            bet: parsed_bet,
        }
    }

    fn rank(&self) -> u32 {
        let combination: Vec<usize> = self
            .cards
            .chars()
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .map(|(_, g)| g.count())
            .sorted()
            .collect();

        match combination.as_slice() {
            [1, 1, 1, 1, 1] => 0,
            [1, 1, 1, 2] => 1,
            [1, 2, 2] => 2,
            [1, 1, 3] => 3,
            [2, 3] => 4,
            [1, 4] => 5,
            [5] => 6,
            _ => panic!("Invalid combination"),
        }
    }
}

fn get_value(c: char) -> u32 {
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap(),
    }
}

fn complex_criterion(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    match a.rank().cmp(&b.rank()) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        std::cmp::Ordering::Equal => a
            .cards
            .chars()
            .zip(b.cards.chars())
            .map(|(a, b)| get_value(a).cmp(&get_value(b)))
            .find(|&ordering| ordering != std::cmp::Ordering::Equal)
            .unwrap_or(std::cmp::Ordering::Equal),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(solution(input), 6440);
    }
}
