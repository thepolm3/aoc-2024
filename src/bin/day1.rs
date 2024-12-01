use anyhow::Result;
use itertools::Itertools;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut array1, mut array2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect_vec();
            (
                parts[0].parse::<u32>().unwrap(),
                parts[1].parse::<u32>().unwrap(),
            )
        })
        .collect();
    array1.sort();
    array2.sort();

    (array1, array2)
}
fn part1(array1: &[u32], array2: &[u32]) -> u32 {
    array1
        .iter()
        .zip(array2)
        .map(|(&a, &b)| a.abs_diff(b))
        .sum()
}

fn part2(array1: &[u32], array2: &[u32]) -> u32 {
    array1
        .iter()
        .map(|x| x * (array2.iter().filter(|&y| *y == *x).count() as u32))
        .sum()
}

fn main() -> Result<()> {
    let (array1, array2) = parse(&std::fs::read_to_string("inputs/day1.txt")?);
    let p1 = part1(&array1, &array2);
    println!("1.1: {p1}");

    let p2 = part2(&array1, &array2);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

    use super::*;

    #[test]
    fn test_part1() {
        let (p1, p2) = parse(INPUT);
        assert_eq!(part1(&p1, &p2), 11);
    }

    #[test]
    fn test_part2() {
        let (p1, p2) = parse(INPUT);
        assert_eq!(part2(&p1, &p2), 31);
    }
}
