use std::{
    collections::{HashMap, HashSet},
    ops::Rem,
};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
struct Antennaes {
    width: usize,
    height: usize,
    inner: HashMap<char, Vec<(usize, usize)>>,
}
fn parse(input: &str) -> Antennaes {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut inner = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                inner.entry(char).or_insert(Vec::new()).push((x, y))
            }
        }
    }
    Antennaes {
        width,
        height,
        inner,
    }
}
fn part1(antennaes: &Antennaes) -> usize {
    let mut antinodes = HashSet::new();
    for (_, positions) in antennaes.inner.iter() {
        for &(x, y) in positions {
            let range = (
                x.min(antennaes.width - x - 1),
                y.min(antennaes.height - y - 1),
            );

            for (ox, oy) in positions.iter().filter(|&&(ox, oy)| {
                !(x == ox && y == oy)
                    && ox <= x + range.0
                    && ox >= x - range.0
                    && oy <= y + range.1
                    && oy >= y - range.1
            }) {
                antinodes.insert((2 * x - ox, 2 * y - oy));
            }
        }
    }
    antinodes.len()
}

fn part2(antennaes: &Antennaes) -> usize {
    let mut antinodes = HashSet::new();
    let explore_iter = (1isize..)
        .map(|x| if x.rem(2) == 0 { x / 2 } else { -x / 2 })
        .take(antennaes.width.max(antennaes.height) * 2);

    for (_, positions) in antennaes.inner.iter() {
        for &(x, y) in positions {
            // println!("({x},{y}), {range:?}");
            for (ox, oy) in positions.iter().filter(|&&(ox, oy)| (ox, oy) > (x, y)) {
                let (x, y, ox, oy) = (x as isize, y as isize, *ox as isize, *oy as isize);
                let diff = (ox - x, oy - y);
                for multiple in explore_iter.clone() {
                    let (cx, cy) = (x + diff.0 * multiple, y + diff.1 * multiple);
                    if cx < 0
                        || cx >= antennaes.width as isize
                        || cy < 0
                        || cy >= antennaes.height as isize
                    {
                        continue;
                    }
                    antinodes.insert((cx, cy));
                }
            }
        }
    }
    antinodes.len()
}

fn main() -> Result<()> {
    let input = parse(&std::fs::read_to_string("inputs/day8.txt")?);
    let p1 = part1(&input);
    println!("1.1: {p1}");

    let p2 = part2(&input);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 14);
    }

    #[test]
    fn test_part2() {
        let input = parse(INPUT);
        assert_eq!(part2(&input), 34);
    }
}
