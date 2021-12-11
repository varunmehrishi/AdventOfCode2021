use std::collections::VecDeque;

use super::octopus::Octopus;

pub fn find_steps<const N: usize, const M: usize>(mut octopi: [[Octopus; M]; N]) -> u32 {
    let mut i = 0;
    loop {
        i += 1;
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

        let mut flashed = 0;

        for octrow in octopi.iter_mut() {
            for octopus in octrow.iter_mut() {
                flashed += octopus.flashed as usize;
                octopus.finalize();
            }
        }

        if flashed == N * M {
            return i;
        }
    }
}
