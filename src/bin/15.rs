#[aoc::main(15)]
fn main(input: &str) -> (u32, u32) {
    let p1: u32 = input.split(',').map(hash_str).sum();

    let mut hm: Vec<(Vec<&str>, Vec<u32>)> = (0..=255)
        .map(|_| {
            let keys: Vec<&str> = Vec::new();
            let values: Vec<u32> = Vec::new();
            (keys, values)
        })
        .collect();

    input.split(',').for_each(|x| {
        if x.contains('-') {
            let key = x.split('-').collect::<Vec<&str>>();
            if let Some(idx) = hm[hash_str(key[0]) as usize]
                .0
                .iter()
                .position(|&x| x == key[0])
            {
                hm[hash_str(key[0]) as usize].1.remove(idx);
                hm[hash_str(key[0]) as usize].0.remove(idx);
            }
        } else {
            let key_val_pair = x.split('=').collect::<Vec<&str>>();
            let key = key_val_pair[0];
            let val = key_val_pair[1].parse::<u32>().unwrap();
            if let Some(idx) = hm[hash_str(key) as usize].0.iter().position(|&x| x == key) {
                hm[hash_str(key) as usize].1[idx] = val;
            } else {
                hm[hash_str(key) as usize].0.push(key);
                hm[hash_str(key) as usize].1.push(val);
            }
        }
    });

    let p2 = hm
        .iter()
        .enumerate()
        .map(|(i, (_, values))| {
            values
                .iter()
                .enumerate()
                .map(|(j, x)| (i as u32 + 1) * *x * (j as u32 + 1))
                .sum::<u32>()
        })
        .sum::<u32>();

    (p1, p2)
}

fn hash_str(input: &str) -> u32 {
    input
        .chars()
        .map(|c| c as u8)
        .fold(0, |acc: u32, c| (acc + c as u32) * 17 % 256)
}
