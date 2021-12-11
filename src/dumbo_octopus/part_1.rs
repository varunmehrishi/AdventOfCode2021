use std::collections::VecDeque;

use super::octopus::Octopus;

pub fn step<const N: usize, const M: usize>(mut octopi: [[Octopus; M]; N], steps: i32) -> u32 {
    for _ in 0..steps {
        let mut flashed_neighbors: VecDeque<(usize, usize)> = VecDeque::new();
        for octrow in octopi.iter_mut() {
            for octopus in octrow.iter_mut() {
                let v = octopus.update((N, M));
                flashed_neighbors.extend(v);
            }
        }

        while let Some((i, j)) = flashed_neighbors.pop_front() {
            let v = octopi[i][j].update((N, M));
            flashed_neighbors.extend(v);
        }

        for octrow in octopi.iter_mut() {
            for octopus in octrow.iter_mut() {
                octopus.finalize();
            }
        }
    }

    octopi
        .iter()
        .map(|octrow| octrow.iter().map(|octopus| octopus.flashes).sum::<u32>())
        .sum::<u32>()
}
