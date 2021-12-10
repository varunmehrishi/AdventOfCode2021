use super::part_1::classify;
use super::SyntaxStatus;

pub fn find_incomplete_syntax_score(lines: &[String]) -> i64 {
    let mut scores: Vec<i64> = lines
        .iter()
        .map(|x| classify(x))
        .filter(|x| matches!(x, SyntaxStatus::Incomplete(_)))
        .map(|x| match x {
            SyntaxStatus::Incomplete(stack) => stack
                .into_iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!("Unreachable Symbol"),
                })
                .fold(0, |acc, e| acc * 5 + e),
            _ => unreachable!("Unreachable SyntaxStatus"),
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}
