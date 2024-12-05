use partial_sort::PartialSort;
use std::cmp::Ordering;
use std::collections::BTreeSet;

use anyhow::Result;
use itertools::chain;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, map_res},
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(PartialEq, Eq)]
enum BinaryTree {
    Empty,
    Full((u32, Box<BinaryTree>, Box<BinaryTree>)),
}

fn digit1(input: &str) -> IResult<&str, u32> {
    map_res(nom::character::complete::digit1, str::parse::<u32>)(input)
}

fn rule(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(digit1, tag("|"), digit1)(input)
}

fn list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), digit1)(input)
}

fn parse<'a>(input: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    separated_pair(
        separated_list1(newline, rule),
        count(newline, 2),
        separated_list1(newline, list),
    )(input)
}

// fn partial_order(rules: &[(u32, u32)]) -> Vec<u32> {
//     let mut sorted = Vec::with_capacity(rules.len() * 2);

//     let mut rules = rules.to_owned();

//     let mut unsorted: Vec<u32> = rules
//         .iter()
//         .flat_map(|&(x, y)| [x, y].into_iter())
//         .collect();

//     while !unsorted.is_empty() {
//         //smallest cannot appear on the right side of any rules
//         let smallest_index = unsorted
//             .iter()
//             .position(|x| rules.iter().all(|(_, b)| x != b))
//             .unwrap();
//         let smallest_value = unsorted.swap_remove(smallest_index);
//         println!("{smallest_value}");
//         sorted.push(smallest_value);

//         //can remove rules containing the smallest
//         rules.retain(|(a, _)| *a != smallest_value);
//     }

//     sorted
// }

fn part1(rules: &[(u32, u32)], lists: &[Vec<u32>]) -> u32 {
    let direct_cmp = |x, y| {
        if rules.contains(&(x, y)) {
            return Some(Ordering::Less);
        }
        if rules.contains(&(y, x)) {
            return Some(Ordering::Greater);
        }
        None
    };

    lists
        .iter()
        .filter(|list| {
            !(0..list.len())
                .flat_map(|i| (i..list.len()).map(move |j| direct_cmp(list[i], list[j])))
                .any(|cmp| cmp == Some(Ordering::Greater))
        })
        .map(|list| list[list.len() / 2])
        .sum()
}

fn sort(mut list: Vec<u32>, rules: &[(u32, u32)]) -> Vec<u32> {
    if list.is_empty() {
        return list;
    }
    let pivot = list.pop().unwrap();
    let mut less = sort(
        list.iter()
            .filter(|y| rules.contains(&(pivot, **y)))
            .copied()
            .collect(),
        rules,
    );
    let greater = sort(
        list.iter()
            .filter(|x| rules.contains(&(**x, pivot)))
            .copied()
            .collect(),
        rules,
    );

    less.push(pivot);
    less.extend(greater);
    less
}
fn part2(rules: &[(u32, u32)], lists: &[Vec<u32>]) -> u32 {
    //stupid bubblesort ass algorithm
    lists
        .iter()
        .map(|list| sort(list.clone(), rules))
        .map(|list| list[list.len() / 2])
        .sum::<u32>()
        - part1(rules, lists)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day5.txt")?;
    let (_, (rules, tests)) = parse(&input).unwrap();

    let p1 = part1(&rules, &tests);
    println!("1.1: {p1}");

    let p2 = part2(&rules, &tests);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    use super::*;

    #[test]
    fn test_part1() {
        let (_, (r, l)) = parse(INPUT).unwrap();
        assert_eq!(part1(&r, &l), 143);
    }

    #[test]
    fn test_part2() {
        let (_, (r, l)) = parse(INPUT).unwrap();
        assert_eq!(part2(&r, &l), 123);
    }
}
