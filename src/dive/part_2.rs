use super::Command;

pub(super) fn find_position(movements: &[Command]) -> (i64, i64) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    movements
        .iter()
        .for_each(|movement: &Command| match movement {
            Command::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Command::Down(x) => aim += x,
            Command::Up(x) => aim -= x,
        });

    (horizontal, depth)
}
