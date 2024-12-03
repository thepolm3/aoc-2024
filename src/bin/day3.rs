use anyhow::Result;
use regex::Regex;

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|x| {
            x.get(1).unwrap().as_str().parse::<u32>().unwrap()
                * x.get(2).unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut acc = 0;
    for capture in re.captures_iter(input) {
        let name = capture.get(0).unwrap().as_str();
        println!("{name}");
        match name {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    acc += capture.get(1).unwrap().as_str().parse::<u32>().unwrap()
                        * capture.get(2).unwrap().as_str().parse::<u32>().unwrap()
                }
            }
        };
    }
    acc
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day3.txt")?;
    let p1 = part1(&input);
    println!("1.1: {p1}");

    let p2 = part2(&input);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2), 48);
    }
}
