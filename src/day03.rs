use std::collections::HashSet;

fn char_to_score(ch: char) -> u32 {
    if ch.is_lowercase() {
        return ch as u32 - 96;
    }

    ch as u32 - 64 + 26
}

pub fn day03a() -> u32 {
    let input = crate::load_to_string("data/day03/day03a.txt").unwrap();
    let mut results = Vec::new();

    for line in input.lines() {
        let split_of_point = line.len() / 2;
        let line_first = HashSet::<char>::from_iter(line[..split_of_point].chars());
        let line_second = HashSet::<char>::from_iter(line[split_of_point..].chars());
        if let Some(ch) = line_first.intersection(&line_second).next() {
            results.push(*ch);
        }
    }

    results.into_iter().map(char_to_score).sum()
}

pub fn day03b() -> u32 {
    let input = crate::load_to_string("data/day03/day03b.txt").unwrap();
    let mut results = Vec::new();
    let mut previous_line_set = HashSet::<char>::new();

    for (line_index, line) in input.lines().enumerate() {
        match line_index % 3 {
            0 => previous_line_set = HashSet::from_iter(line.chars()),
            1 => {
                let line_set = HashSet::from_iter(line.chars());
                previous_line_set = line_set.intersection(&previous_line_set).copied().collect();
            }
            2 => {
                let line_set = HashSet::from_iter(line.chars());
                results.extend(line_set.intersection(&previous_line_set).copied());
            }
            _ => unreachable!(),
        }
    }

    results.into_iter().map(char_to_score).sum()
}
