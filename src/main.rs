use std::time::{Duration, Instant};

use crate::{bounded_u8::UBoundU8, grid::Grid, solver::solve_sudoku};

mod bounded_u8;
mod grid;
mod puzzle;
mod solver;
mod sort;

fn load_puzzles() -> Vec<[UBoundU8<9>; 81]> {
    let file_contents = std::fs::read_to_string("puzzles.txt").unwrap();

    file_contents
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
        .map(|line| {
            let mut cells: [UBoundU8<9>; 81] = [UBoundU8::new(0); 81];

            if line.len() != 81 {
                panic!(
                    "Error loading puzzle. Line had {} bytes, expected 81",
                    line.len()
                );
            }

            for (i, byte) in line.bytes().enumerate() {
                if (b'0'..=b'9').contains(&byte) {
                    cells[i] = UBoundU8::new(byte - b'0');
                }
            }

            cells
        })
        .collect()
}

fn main() {
    let mut total_solve_time = Duration::ZERO;
    let mut num_puzzles = 0;

    for (i, puzzle) in load_puzzles().into_iter().enumerate() {
        let start = Instant::now();
        let grid = Grid::new(puzzle);
        let solution = solve_sudoku(&grid);
        let elapsed = start.elapsed();

        total_solve_time += elapsed;
        num_puzzles += 1;

        match solution {
            Some(_grid) => println!("Solved puzzle {} in {}us", i, elapsed.as_micros()),
            None => panic!("Failed to solve puzzle {}", i),
        }
    }

    println!(
        "Solved {} puzzles in {}s (average: {}ms)",
        num_puzzles,
        total_solve_time.as_secs_f64(),
        (total_solve_time / num_puzzles).as_micros() as f64 / 1000.0
    );
}
