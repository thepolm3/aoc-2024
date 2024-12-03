use anyhow::Result;
use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn row_is_safe(row: &[u32]) -> bool {
    let ordering = row[0].cmp(&row[1]);
    row.iter().tuple_windows().all(|(&x, &y)| {
        let diff = x.abs_diff(y);
        (1..=3).contains(&diff) && x.cmp(&y) == ordering
    })
}
/** Safe inputs */
fn part1(array: &[Vec<u32>]) -> u32 {
    array.iter().filter(|row| row_is_safe(row)).count() as u32
}

fn part2(array: &[Vec<u32>]) -> u32 {
    array
        .iter()
        .filter(|row| {
            let mut is_safe = row_is_safe(row);
            for index in 0..row.len() {
                let mut removed = (*row).clone();
                removed.remove(index);
                is_safe |= row_is_safe(&removed);
            }
            is_safe
        })
        .count() as u32
}

fn main() -> Result<()> {
    let array = parse(&std::fs::read_to_string("inputs/day2.txt")?);
    let p1 = part1(&array);
    println!("1.1: {p1}");

    let p2 = part2(&array);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 4);
    }
}
