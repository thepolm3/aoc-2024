use std::ops::Rem;

use anyhow::Result;
use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map_res,
    multi::separated_list1, sequence::separated_pair, IResult,
};

fn digit1(input: &str) -> IResult<&str, u64> {
    map_res(nom::character::complete::digit1, str::parse::<u64>)(input)
}

fn rhs(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), digit1)(input)
}
fn equation(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(digit1, tag(": "), rhs)(input)
}
fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(newline, equation)(input)
}

//assumes the components are reversed
fn can_be_made(target: u64, components: &[u64]) -> bool {
    if components.is_empty() {
        return target == 0;
    }
    if target < components[0] {
        return false;
    }
    let by_addition = can_be_made(target - components[0], &components[1..]);

    let by_multiplication =
        target.rem(components[0]) == 0 && can_be_made(target / components[0], &components[1..]);

    by_addition || by_multiplication
}

//assumes the components are reversed
fn can_be_made_p2(target: u64, components: &[u64]) -> bool {
    if components.is_empty() {
        return target == 0;
    }

    if target < components[0] {
        return false;
    }

    if can_be_made_p2(target - components[0], &components[1..]) {
        return true;
    }

    if target.rem(components[0]) == 0 && can_be_made_p2(target / components[0], &components[1..]) {
        return true;
    }

    if let Some(new_target) = target.to_string().strip_suffix(&components[0].to_string()) {
        return can_be_made_p2(
            new_target.parse::<u64>().unwrap_or_default(),
            &components[1..],
        );
    };

    false
}

fn part1(input: Vec<(u64, Vec<u64>)>) -> u64 {
    let mut result = 0;
    for (target, mut components) in input {
        components.reverse();
        if can_be_made(target, &components) {
            println!("possible: {components:?} ({target})");
            result += target
        } else {
            println!("impossible: {components:?} ({target})");
        }
    }
    result
}

fn part2(input: Vec<(u64, Vec<u64>)>) -> u64 {
    let mut result = 0;
    for (target, mut components) in input {
        components.reverse();
        if can_be_made_p2(target, &components) {
            println!("possible: {components:?} ({target})");
            result += target
        } else {
            println!("impossible: {components:?} ({target})");
        }
    }
    result
}

fn main() -> Result<()> {
    let (_, input) = parse(&std::fs::read_to_string("inputs/day7.txt")?).unwrap();
    println!("{input:?}");
    let p1 = part1(input.clone());
    println!("1.1: {p1}");

    let p2 = part2(input);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    use super::*;

    #[test]
    fn test_part1() {
        let (_, input) = parse(INPUT).unwrap();
        assert_eq!(part1(input), 3749);
    }

    #[test]
    fn test_part2() {
        let (_, input) = parse(INPUT).unwrap();
        assert_eq!(part2(input), 11387);
    }
}
