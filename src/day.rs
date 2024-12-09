use anyhow::Result;
use itertools::Itertools;

fn parse(input: &str) ->  {}
fn part1(input: ()) -> u32 {
    0
}

fn part2(input: ()) -> u32 {
    0
}

fn main() -> Result<()> {
    let input = parse(&std::fs::read_to_string("inputs/day.txt")?);
    let p1 = part1(input);
    println!("1.1: {p1}");

    let p2 = part2(input);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "";

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(INPUT);
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = parse(INPUT);
        assert_eq!(part2(input), 0);
    }
}
