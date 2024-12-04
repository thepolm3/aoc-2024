use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Grid<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid<char> {
    fn from_str(s: &str) -> Grid<char> {
        let inner = s.lines().collect_vec();
        let width = inner.first().unwrap_or(&"").len();
        let height = inner.len();
        assert!(inner.iter().all(|s| s.len() == width));

        Grid {
            inner: inner.iter().flat_map(|line| line.chars()).collect(),
            width,
            height,
        }
    }

    fn find_word_at(&self, word: &str, x: isize, y: isize) -> u32 {
        let len = word.len() as isize;

        //love finding 0 length words everywhere
        if len == 0 {
            return 8;
        }

        //quick check for first letter
        if self.get_isize(x, y) != word.chars().nth(0) {
            return 0;
        }
        [
            [1, 0],
            [1, 1],
            [0, 1],
            [-1, 1],
            [-1, 0],
            [-1, -1],
            [0, -1],
            [1, -1],
        ]
        .iter()
        .filter(|[dx, dy]| {
            (1..len).all(|i| self.get_isize(x + dx * i, y + dy * i) == word.chars().nth(i as usize))
        })
        .count() as u32
    }
}
impl<T> Grid<T>
where
    T: Copy,
{
    fn get(&self, x: usize, y: usize) -> Option<T> {
        (x < self.width && y < self.height)
            .then(|| self.inner.get(x + y * self.width).copied())
            .flatten()
    }

    fn get_isize(&self, x: isize, y: isize) -> Option<T> {
        (x >= 0 && y >= 0)
            .then(|| self.get(x as usize, y as usize))
            .flatten()
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
    T: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                self.get(x, y).unwrap().fmt(f)?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Grid<char> {
    Grid::from_str(input)
}
fn part1(grid: &Grid<char>, search: &str) -> u32 {
    (0..grid.height)
        .cartesian_product(0..grid.width)
        .map(|(y, x)| grid.find_word_at(search, x as isize, y as isize))
        .sum()
}

fn part2(grid: Grid<char>) -> u32 {
    (0..grid.height as isize)
        .cartesian_product(0..grid.width as isize)
        .filter(|(y, x)| {
            if Some('A') != grid.get_isize(*x, *y) {
                return false;
            };
            let nw = grid.get_isize(x - 1, y - 1);
            let se = grid.get_isize(x + 1, y + 1);
            if !(matches!((nw, se), (Some('M'), Some('S')) | (Some('S'), Some('M')))) {
                return false;
            }
            let ne = grid.get_isize(x + 1, y - 1);
            let sw = grid.get_isize(x - 1, y + 1);
            if !(matches!((ne, sw), (Some('M'), Some('S')) | (Some('S'), Some('M')))) {
                return false;
            }
            true
        })
        .count() as u32
}

fn main() -> Result<()> {
    let input = parse(&std::fs::read_to_string("inputs/day4.txt")?);

    let p1 = part1(&input, "XMAS");
    println!("1.1: {p1}");

    let p2 = part2(input);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(INPUT);
        assert_eq!(part1(&input, "XMAS"), 18);
    }

    #[test]
    fn test_part2() {
        let input = parse(INPUT);
        assert_eq!(part2(input), 9);
    }
}
