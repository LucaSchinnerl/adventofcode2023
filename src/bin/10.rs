use geo::Contains;
use geo::{point, LineString, Polygon};

#[aoc::main(10)]
fn main(input: &str) -> (u32, u32) {
    let char_map = input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (steps, path) = solution(&char_map);

    let ls = LineString::from(path);

    let polygon = Polygon::new(ls, vec![]);
    let mut area = 0;
    char_map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, _)| {
            if polygon.contains(&point!(x: x as f64, y: y as f64)) {
                area += 1;
            }
        })
    });

    (steps, area)
}

fn solution(map: &[Vec<char>]) -> (u32, Vec<(f64, f64)>) {
    let mut coordinate_path = Vec::new();

    let start_position = find_start_position(map);
    coordinate_path.push(start_position);
    let mut prev = start_position;
    let mut current_position = get_starting_direction(start_position, map);
    let mut steps = 1;

    loop {
        coordinate_path.push(current_position);
        if current_position == start_position {
            break;
        }

        let c = map[current_position.1][current_position.0];
        let connections = get_connections(c, current_position.0, current_position.1);
        let next = filter_next_position(connections, &prev);
        prev = current_position;
        current_position = next;
        steps += 1;
    }
    (
        steps / 2,
        coordinate_path
            .iter()
            .map(|(x, y)| (*x as f64, *y as f64))
            .collect(),
    )
}

fn find_start_position(map: &[Vec<char>]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap()
}

fn get_starting_direction(starting_position: (usize, usize), map: &[Vec<char>]) -> (usize, usize) {
    let (x, y) = starting_position;
    for direction in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (dx, dy) = direction;
        let next_position = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
        let c = map[next_position.1][next_position.0];
        let connections = get_connections(c, next_position.0, next_position.1);
        if connections.contains(&starting_position) {
            return next_position;
        }
    }
    panic!("Invalid starting position")
}

fn get_connections(c: char, x: usize, y: usize) -> Vec<(usize, usize)> {
    match c {
        '|' => vec![(x, y + 1), (x, y.saturating_sub(1))],
        '-' => vec![(x + 1, y), (x.saturating_sub(1), y)],
        'L' => vec![(x, y.saturating_sub(1)), (x + 1, y)],
        'J' => vec![(x, y.saturating_sub(1)), (x.saturating_sub(1), y)],
        '7' => vec![(x, y + 1), (x.saturating_sub(1), y)],
        'F' => vec![(x, y + 1), (x + 1, y)],
        _ => {
            println!("{}", c);
            panic!("Invalid character")
        }
    }
}

fn filter_next_position(next: Vec<(usize, usize)>, prev: &(usize, usize)) -> (usize, usize) {
    next.into_iter().find(|(x, y)| &(*x, *y) != prev).unwrap()
}
