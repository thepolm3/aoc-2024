use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Obstacle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Guard {
    position: (isize, isize),
    heading: (isize, isize),
}

impl Guard {
    fn next_step(&self) -> (isize, isize) {
        (
            self.position.0 + self.heading.0,
            self.position.1 + self.heading.1,
        )
    }

    fn step(&mut self) {
        self.position = self.next_step()
    }

    fn turn(&mut self) {
        self.heading = (-self.heading.1, self.heading.0)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "-")?;
        }
        writeln!(f, "+")?;
        for line in self.cells.iter().chunks(self.width).into_iter() {
            write!(f, "|")?;
            for cell in line {
                write!(
                    f,
                    "{}",
                    match cell {
                        Cell::Obstacle => "#",
                        Cell::Empty => ".",
                    }
                )?
            }
            writeln!(f, "|")?;
        }
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "-")?;
        }
        writeln!(f, "+")
    }
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        (x < self.width && y < self.height)
            .then(|| self.cells.get(x + y * self.width).copied())
            .flatten()
    }

    fn get_isize(&self, x: isize, y: isize) -> Option<Cell> {
        (x >= 0 && y >= 0)
            .then(|| self.get(x as usize, y as usize))
            .flatten()
    }

    //does nothing if out of range
    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[x + y * self.width] = cell;
        }
    }

    fn set_isize(&mut self, x: isize, y: isize, cell: Cell) {
        if x >= 0 && y >= 0 {
            self.set(x as usize, y as usize, cell)
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        (x < self.width && y < self.height)
            .then(|| self.cells.get_mut(x + y * self.width))
            .flatten()
    }
}

fn parse(input: &str) -> (Grid, Guard) {
    let input = input.lines().collect_vec();
    let width = input.first().unwrap_or(&"").len();
    let height = input.len();
    let position = input
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars()
                .position(|c| c == '^')
                .map(|x| (x as isize, y as isize))
        })
        .unwrap();

    (
        Grid {
            cells: input
                .into_iter()
                .flat_map(|line| {
                    line.chars().map(|c| match c {
                        '#' => Cell::Obstacle,
                        _ => Cell::Empty,
                    })
                })
                .collect(),
            width,
            height,
        },
        Guard {
            position,
            heading: (0, -1),
        },
    )
}
fn part1(grid: &Grid, mut guard: Guard) -> u32 {
    let mut visited = HashSet::with_capacity(grid.cells.len());
    loop {
        visited.insert(guard.position);
        let (x, y) = guard.next_step();
        match grid.get_isize(x, y) {
            Some(Cell::Empty) => guard.step(),
            Some(Cell::Obstacle) => guard.turn(),
            None => break,
        }
    }
    visited.len() as u32
}

fn loops(grid: &Grid, mut guard: Guard) -> bool {
    let mut visited = HashSet::with_capacity(grid.cells.len());
    loop {
        if visited.contains(&guard) {
            return true;
        }
        visited.insert(guard);
        let (x, y) = guard.next_step();
        match grid.get_isize(x, y) {
            Some(Cell::Empty) => guard.step(),
            Some(Cell::Obstacle) => guard.turn(),
            None => return false,
        }
    }
}

fn part2(mut grid: Grid, guard: Guard) -> u32 {
    let w = grid.width;
    let h = grid.height;
    let mut n_loops = 0;
    for (x, y) in (0..w).flat_map(|x| (0..h).map(move |y| (x, y))) {
        let is_empty = matches!(grid.get(x, y), Some(Cell::Empty));
        if is_empty {
            grid.set(x, y, Cell::Obstacle);
            if loops(&grid, guard) {
                n_loops += 1
            }
            grid.set(x, y, Cell::Empty);
        }
    }
    n_loops
}

fn main() -> Result<()> {
    let (grid, guard) = parse(&std::fs::read_to_string("inputs/day6.txt")?);

    let p1 = part1(&grid, guard);
    println!("1.1: {p1}");

    let p2 = part2(grid, guard);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    use super::*;

    #[test]
    fn test_part1() {
        let (grid, guard) = parse(INPUT);
        assert_eq!(part1(&grid, guard), 41);
    }

    #[test]
    fn test_part2() {
        let (grid, guard) = parse(INPUT);
        assert_eq!(part2(grid, guard), 6);
    }
}
