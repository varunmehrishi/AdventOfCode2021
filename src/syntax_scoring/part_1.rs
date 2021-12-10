use super::SyntaxStatus;

pub(super) fn classify(s: &str) -> SyntaxStatus {
    let mut stack = vec![];

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
            }
            _ => {
                match (stack.last(), c) {
                    (Some('('), ')') => {
                        stack.pop();
                    }
                    (Some('['), ']') => {
                        stack.pop();
                    }
                    (Some('{'), '}') => {
                        stack.pop();
                    }
                    (Some('<'), '>') => {
                        stack.pop();
                    }
                    v => return SyntaxStatus::Corrupt(v.1),
                };
            }
        }
    }

    if stack.is_empty() {
        SyntaxStatus::Correct
    } else {
        SyntaxStatus::Incomplete(stack)
    }
}

pub fn find_corrupted_syntax_score(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|x| classify(x))
        .filter(|x| matches!(x, SyntaxStatus::Corrupt(_)))
        .map(|x| match x {
            SyntaxStatus::Corrupt(c) => c,
            _ => unreachable!("unreachable"),
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!("unreachable"),
        })
        .sum()
}
