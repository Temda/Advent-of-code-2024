

fn main() {
    let map = include_bytes!("../input.txt")
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();

    part_1(&map);
    part_2(&map);
}

fn part_1(map: &[&[u8]]) {
    let mut word = [0; 4];
    let count = (0..map[0].len() as isize)
        .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
        .flat_map(|(x, y)| {
            [
                [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)], // NE
                [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],             // E
                [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)], // SE
                [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],             // S
            ]
        })
        .filter(|coords| {
            let mut iter = coords.iter().map(|(x, y)| {
                map.get(*y as usize)
                    .and_then(|row| row.get(*x as usize).copied())
                    .unwrap_or_default()
            });
            word.fill_with(|| iter.next().unwrap_or_default());
            &word == b"XMAS" || &word == b"SAMX"
        })
        .count();

    println!("Part 1: The word 'XMAS' or 'SAMX' appears {} times.", count);
}

fn part_2(map: &[&[u8]]) {
    let mut cross = [0; 4];

    let count = (0..map[0].len() as isize)
        .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
        .map(|(x, y)| {
            [
                (x + 1, y + 1), // Center
                (x, y),         // NE
                (x, y + 2),     // SE
                (x + 2, y),     // NW
                (x + 2, y + 2), // SW
            ]
        })
        .filter(|coords| {
            let mut iter = coords.iter().map(|(x, y)| {
                map.get(*y as usize)
                    .and_then(|row| row.get(*x as usize).copied())
                    .unwrap_or_default()
            });

            if iter.next().is_none_or(|n| n != b'A') {
                return false;
            }

            cross[0] = iter.next().unwrap_or_default();
            cross[1] = iter.next().unwrap_or_default();
            cross[2] = iter.next().unwrap_or_default();
            cross[3] = iter.next().unwrap_or_default();

            &cross == b"MMSS" || &cross == b"MSMS" || &cross == b"SSMM" || &cross == b"SMSM"
        })
        .count();

    println!("Part 2: The X-MAS pattern appears {} times.", count);
}