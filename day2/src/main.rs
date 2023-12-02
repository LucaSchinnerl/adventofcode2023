use std::cmp::max;

fn main() {
    let input = include_str!("../input.txt");

    println!("{:?}", solution(input));
}

struct RGB {
    blue: u32,
    red: u32,
    green: u32,
}

impl RGB {
    fn new() -> RGB {
        RGB {
            blue: 0,
            red: 0,
            green: 0,
        }
    }

    fn max(&mut self, color: &str, number: u32) {
        match color {
            "blue" => self.blue = max(self.blue, number),
            "red" => self.red = max(self.red, number),
            "green" => self.green = max(self.green, number),
            _ => panic!("Invalid color"),
        }
    }

    fn is_valid(&self) -> bool {
        self.blue <= 14 && self.red <= 12 && self.green <= 13
    }

    fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

fn max_cubes(input: &str) -> RGB {
    let games = input.split(": ").nth(1).unwrap().replace(';', ",");
    let mut rgb = RGB::new();

    games
        .split(", ")
        .map(|x| {
            let t = x.split(' ').collect::<Vec<&str>>();
            (t[1], t[0].parse::<u32>().unwrap())
        })
        .for_each(|(color, number)| {
           rgb.max(color, number)
        });
    rgb
}

fn parse_game_id(input: &str) -> u32 {
    return input.split(':').collect::<Vec<&str>>()[0]
        .split(' ')
        .collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();
}

fn solution(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;

    input
        .lines()
        .for_each(|x| {
            let rgb = max_cubes(x);
            if rgb.is_valid() {
                part1 += parse_game_id(x);
            }
            part2 += rgb.power();
        });
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solution(input), (8, 2286));
    }
}
