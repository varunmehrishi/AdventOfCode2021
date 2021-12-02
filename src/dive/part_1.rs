use super::Command;

pub(super) fn find_position(movements: &[Command]) -> (i64, i64) {
    let mut horizontal = 0;
    let mut depth = 0;

    movements
        .iter()
        .for_each(|movement: &Command| match movement {
            Command::Forward(x) => horizontal += x,
            Command::Down(x) => depth += x,
            Command::Up(x) => depth -= x,
        });

    (horizontal, depth)
}
