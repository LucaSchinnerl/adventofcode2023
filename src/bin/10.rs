#[aoc::main(10)]
fn main(input: &str) -> (u32, u32) {
    let char_map = input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (steps, _map) = solution(&char_map);

    (steps, 0)
}

fn solution(map: &[Vec<char>]) -> (u32, Vec<Vec<u8>>) {
    let mut map_path: Vec<Vec<u8>> = map
        .iter()
        .map(|row| row.iter().map(|_| 0).collect())
        .collect();

    let start_position = find_start_position(map);
    map_path[start_position.1][start_position.0] = 1;
    let mut prev = start_position;
    let mut current_position = get_starting_direction(start_position, map);
    let mut steps = 1;

    loop {
        map_path[current_position.1][current_position.0] = 1;
        let (x, y) = current_position;
        let c = map[y][x];
        let connections = get_connections(c, x, y);
        let next = filter_next_position(connections, &prev);
        prev = current_position;
        current_position = next;
        steps += 1;
        if current_position == start_position {
            break;
        }
    }
    (steps / 2, map_path)
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
