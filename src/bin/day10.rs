use std::{collections::HashSet, convert::identity};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cell {
    val: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "-")?;
        }
        writeln!(f, "+")?;
        for line in self.cells.iter().chunks(self.width).into_iter() {
            write!(f, "|")?;
            for cell in line {
                write!(f, "{}", cell.val)?
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
        self.in_bounds(x, y)
            .then(|| self.cells.get(x + y * self.width).copied())
            .flatten()
    }

    fn get_isize(&self, x: isize, y: isize) -> Option<Cell> {
        (x >= 0 && y >= 0)
            .then(|| self.get(x as usize, y as usize))
            .flatten()
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn in_bounds_isize(&self, x: isize, y: isize) -> bool {
        (x >= 0 && y >= 0) && self.in_bounds(x as usize, y as usize)
    }

    fn nbrs(&self, x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> + '_ {
        [(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)]
            .into_iter()
            .filter(|(x, y)| self.in_bounds_isize(*x, *y))
    }

    fn enumerate(&self) -> impl Iterator<Item = ((usize, usize), &Cell)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, c)| ((i % self.width, i / self.width), c))
    }
}

fn parse(input: &str) -> Grid {
    let input = input.lines().collect_vec();
    let width = input.first().unwrap_or(&"").len();
    let height = input.len();
    Grid {
        cells: input
            .into_iter()
            .flat_map(|line| {
                line.chars().map(|c| Cell {
                    val: c.to_string().parse::<u8>().unwrap(),
                })
            })
            .collect(),
        width,
        height,
    }
}

fn _ascending_paths(grid: &Grid, (x, y): (isize, isize), out: &mut HashSet<(isize, isize)>) {
    let val = grid.get_isize(x, y).unwrap().val;
    if val == 9 {
        out.insert((x, y));
        return;
    }
    for nbr in grid
        .nbrs(x, y)
        .filter(|&(x, y)| grid.get_isize(x, y) == Some(Cell { val: val + 1 }))
    {
        _ascending_paths(grid, nbr, out);
    }
}

fn ascending_paths(grid: &Grid, (x, y): (usize, usize)) -> usize {
    let mut map = HashSet::new();
    _ascending_paths(grid, (x as isize, y as isize), &mut map);
    map.len()
}

fn part1(grid: &Grid) -> usize {
    grid.enumerate()
        .filter(|(_, cell)| cell.val == 0)
        .map(|(xy, _)| ascending_paths(grid, xy))
        .sum()
}

fn trailhead_ratings(grid: &Grid, (x, y): (isize, isize)) -> usize {
    let val = grid.get_isize(x, y).unwrap().val;
    if val == 9 {
        return 1;
    }
    grid.nbrs(x, y)
        .filter(|&(x, y)| grid.get_isize(x, y) == Some(Cell { val: val + 1 }))
        .map(|nbr| trailhead_ratings(grid, nbr))
        .sum()
}

fn part2(grid: &Grid) -> usize {
    grid.enumerate()
        .filter(|(_, cell)| cell.val == 0)
        .map(|((x, y), _)| trailhead_ratings(grid, (x as isize, y as isize)))
        .sum()
}

fn main() -> Result<()> {
    let input = parse(&std::fs::read_to_string("inputs/day10.txt")?);
    let p1 = part1(&input);
    println!("1.1: {p1}");

    let p2 = part2(&input);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    use super::*;

    #[test]
    fn test_part1() {
        let grid = parse(INPUT);
        assert_eq!(part1(&grid), 36);
    }

    #[test]
    fn test_part2() {
        let input = parse(INPUT);
        assert_eq!(part2(&input), 81);
    }
}
