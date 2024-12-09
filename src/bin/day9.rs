use std::fmt::Write;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct FileOrSpace {
    id: Option<usize>,
    length: usize,
}

impl FileOrSpace {
    fn is_file(&self) -> bool {
        self.id.is_some()
    }

    fn is_space(&self) -> bool {
        self.id.is_none()
    }
}

fn print_fs(fs: &[FileOrSpace]) {
    let mut s = String::new();
    for elem in fs.iter() {
        write!(
            s,
            "{}",
            elem.id
                .map(|s| s.to_string())
                .unwrap_or(".".to_owned())
                .repeat(elem.length)
        );
    }
    println!("{s}")
}

// sum of n numbers from "from" of length "length"
// e.g triangle_run(0, 5) = 0 + 1 + 2 + 3 + 4
fn triangle_run(from: usize, length: usize) -> usize {
    let to = from + length;
    if length == 0 {
        return 0;
    }
    if from == 0 {
        return to * (to - 1) / 2;
    }
    ((to * (to - 1)) - (from * (from - 1))) / 2
}
fn parse(input: &str) -> Vec<FileOrSpace> {
    input
        .chars()
        .enumerate()
        .map(|(i, x)| match i % 2 {
            0 => FileOrSpace {
                id: Some(i / 2),
                length: x.to_string().parse::<usize>().unwrap(),
            },
            1 => FileOrSpace {
                id: None,
                length: x.to_string().parse::<usize>().unwrap(),
            },
            _ => unreachable!(),
        })
        .collect()
}

fn calculate_checksum(files: &[FileOrSpace]) -> usize {
    files
        .iter()
        .fold((0, 0), |(len, total), file| {
            if file.is_file() {
                (
                    len + file.length,
                    (total + triangle_run(len, file.length) * file.id.unwrap()),
                )
            } else {
                (len + file.length, total)
            }
        })
        .1
}
fn part1(mut files: Vec<FileOrSpace>) -> usize {
    'outer: loop {
        let mut file = files.pop().unwrap();
        if file.is_space() {
            continue;
        }
        'inner: while file.length > 0 {
            let i = files.iter().position(|x| x.is_space());
            if let Some(i) = i {
                let length = files[i].length;
                if length <= file.length {
                    files[i].id = file.id;
                    file.length -= length;
                } else {
                    files[i].length -= file.length;
                    files.insert(i, file);
                    break 'inner;
                }
            } else {
                files.push(file);
                break 'outer;
            }
        }
    }
    calculate_checksum(&files)
}

fn merge_files(files: Vec<FileOrSpace>) -> Vec<FileOrSpace> {
    files
        .into_iter()
        .chunk_by(|file| file.id)
        .into_iter()
        .map(|(id, chunk)| FileOrSpace {
            id,
            length: chunk.map(|f| f.length).sum(),
        })
        .collect()
}
fn part2(mut files: Vec<FileOrSpace>) -> usize {
    let mut moving_id = files[files.len() - 1]
        .id
        .unwrap_or_else(|| files[files.len() - 2].id.unwrap());

    while moving_id > 0 {
        let file_index = files
            .iter()
            .position(|file| file.id == Some(moving_id))
            .unwrap();

        let file = files[file_index];

        let space_index = files[..file_index]
            .iter()
            .position(|x| x.is_space() && x.length >= file.length);

        if let Some(space_index) = space_index {
            files[space_index].length -= file.length;
            files[file_index].id = None;
            files.insert(space_index, file);
        }
        moving_id -= 1;
    }
    calculate_checksum(&files)
}

fn main() -> Result<()> {
    let input = parse(&std::fs::read_to_string("inputs/day9.txt")?);
    let p1 = part1(input.clone());
    println!("1.1: {p1}");

    let p2 = part2(input);
    println!("1.2: {p2}");

    Ok(())
}
#[cfg(test)]
mod test {
    const INPUT: &str = "2333133121414131402";

    use super::*;

    #[test]
    fn test_triangle_run() {
        assert_eq!(triangle_run(0, 5), 10);
        assert_eq!(triangle_run(1, 4), 10);
        assert_eq!(triangle_run(2, 3), 9);
        assert_eq!(triangle_run(3, 2), 7);
    }
    #[test]
    fn test_part1() {
        let input = parse(INPUT);
        assert_eq!(part1(input), 1928);
    }

    #[test]
    fn test_part2() {
        let input = parse(INPUT);
        assert_eq!(part2(input), 2858);
    }
}
