use itertools::Itertools;
use std::usize;

#[aoc::main(07)]
fn main(input: &str) -> (u32, u32) {
    (solution(input, false), solution(input, true))
}

fn solution(input: &str, part2: bool) -> u32 {
    let mut sorted_hands = input.lines().map(Hand::new).collect::<Vec<Hand>>();
    sorted_hands.sort_by(|a, b| complex_criterion(a, b, part2));
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

    fn rank(&self, part2: bool) -> u32 {
        let mut cards = self.cards.clone();
        if part2 {
            if self.cards == "JJJJJ" {
                cards = "AAAAA".to_string()
            } else {
                let most_common_char = sort_string_with_custom_values(
                    self.cards.chars().filter(|&c| c != 'J').collect::<String>(),
                    part2,
                )
                .chars()
                .max_by_key(|c| self.cards.chars().filter(|&x| x == *c).count())
                .unwrap();

                cards = self.cards.replace('J', &most_common_char.to_string());
            }
        }

        let combination: Vec<usize> = cards
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

fn sort_string_with_custom_values(s: String, part2: bool) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by_key(|a| get_value(*a, part2));
    chars.into_iter().collect()
}

fn get_value(c: char, part2: bool) -> u32 {
    match c {
        'T' => 10,
        'J' => {
            if part2 {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap(),
    }
}

fn complex_criterion(a: &Hand, b: &Hand, part2: bool) -> std::cmp::Ordering {
    match a.rank(part2).cmp(&b.rank(part2)) {
        std::cmp::Ordering::Equal => a
            .cards
            .chars()
            .zip(b.cards.chars())
            .map(|(a, b)| get_value(a, part2).cmp(&get_value(b, part2)))
            .find(|&ordering| ordering != std::cmp::Ordering::Equal)
            .unwrap(),
        _ => a.rank(part2).cmp(&b.rank(part2)),
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
        assert_eq!(solution(input, false), 6440);

        assert_eq!(solution(input, true), 5905);
    }
}
