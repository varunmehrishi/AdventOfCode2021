use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

fn apply_rules(
    template: &HashMap<(char, char), i64>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), i64> {
    template.iter().fold(HashMap::new(), |mut map, (p, count)| {
        match rules.get(p) {
            Some(c) => {
                *map.entry((p.0, *c)).or_insert(0) += count;
                *map.entry((*c, p.1)).or_insert(0) += count;
            }
            _ => {
                *map.entry(*p).or_insert(0) += count;
            }
        }
        map
    })
}

fn get_freqs(template: &HashMap<(char, char), i64>) -> HashMap<char, i64> {
    template.iter().fold(HashMap::new(), |mut map, (p, count)| {
        *map.entry(p.0).or_insert(0) += count;
        *map.entry(p.1).or_insert(0) += count;
        map
    })
}

pub fn find_answer(template: &[char], rules: &HashMap<(char, char), char>) -> i64 {
    let mut map: HashMap<(char, char), i64> =
        template
            .to_vec()
            .windows(2)
            .fold(HashMap::new(), |mut map, w| {
                *map.entry((w[0], w[1])).or_insert(0) += 1;
                map
            });

    for _ in 0..40 {
        map = apply_rules(&map, rules);
    }

    let freq = {
        let mut freq = get_freqs(&map);
        *freq.entry(*template.first().unwrap()).or_default() += 1;
        *freq.entry(*template.last().unwrap()).or_default() += 1;
        freq
    };

    if let MinMaxResult::MinMax(mn, mx) = freq.values().into_iter().map(|x| x / 2).minmax() {
        mx - mn
    } else {
        0
    }
}
