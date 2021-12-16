use core::panic;
use std::{
    collections::{BTreeSet, HashMap},
    path::Path,
};

mod input;

pub fn solve<const N: usize, const M: usize>(file_path: &Path) {
    let matrix = input::read_matrix::<N, M>(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));

    let dist = dijkstra(&matrix, (0, 0), (N as i32 - 1, M as i32 - 1));
    println!("Chiton Part 1: {}", dist.unwrap());

    let tiled_matrix = get_tiled_matrix::<N, M>(&matrix, 5, 5);

    let dist = dijkstra(&tiled_matrix, (0, 0), (5 * N as i32 - 1, 5 * M as i32 - 1));
    println!("Chiton Part 2: {}", dist.unwrap());
}

fn get_tiled_matrix<const N: usize, const M: usize>(
    matrix: &HashMap<(i32, i32), u32>,
    n: i32,
    m: i32,
) -> HashMap<(i32, i32), u32> {
    let mut bmatrix = HashMap::new();
    for i in 0..N as i32 {
        for j in 0..M as i32 {
            for di in 0..n {
                for dj in 0..m {
                    let value = matrix.get(&(i, j)).unwrap() + (di + dj) as u32;

                    bmatrix.insert(
                        (di * N as i32 + i, dj * M as i32 + j),
                        if value > 9 { value - 9 } else { value },
                    );
                }
            }
        }
    }
    bmatrix
}

fn dijkstra(matrix: &HashMap<(i32, i32), u32>, src: (i32, i32), dest: (i32, i32)) -> Option<u32> {
    let mut dist = HashMap::new();
    dist.insert(src, 0);

    let mut non_finalized = BTreeSet::new();
    non_finalized.insert((0, src));

    while let Some((ds, (x, y))) = non_finalized.pop_first() {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let x = x + dx;
            let y = y + dy;

            if matrix.contains_key(&(x, y)) {
                let dd: &mut u32 = dist.entry((x, y)).or_insert(u32::MAX);
                let edge_w = *matrix.get(&(x, y)).unwrap();
                if *dd > ds + edge_w {
                    non_finalized.remove(&(*dd, (x, y)));
                    *dd = ds + edge_w;
                    non_finalized.insert((*dd, (x, y)));
                }
            }
        }
    }

    dist.get(&dest).copied()
}
