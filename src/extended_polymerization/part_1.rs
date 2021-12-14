use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

fn apply_rules(template: &[char], rules: &HashMap<(char, char), char>) -> Vec<char> {
    let first = template.iter().next().unwrap();

    template.iter().skip(1).fold(vec![*first], |mut acc, c| {
        match rules.get(&(*acc.last().unwrap(), *c)) {
            Some(o) => {
                acc.push(*o);
                acc.push(*c);
            }
            _ => acc.push(*c),
        }
        acc
    })
}

pub fn find_answer(template: &[char], rules: &HashMap<(char, char), char>) -> i64 {
    let mut c = template.to_vec();
    for _ in 0..10 {
        c = apply_rules(&c, rules);
    }

    let freq = c.iter().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    if let MinMaxResult::MinMax(mn, mx) = freq.values().into_iter().minmax() {
        mx - mn
    } else {
        0
    }
}
