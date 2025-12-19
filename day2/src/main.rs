use std::fs::File;
use std::io::{BufReader, Read};

fn is_twice_numbers(n: u64) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let n = bytes.len();
    if !n.is_multiple_of(2) {
        return false;
    }
    // println!("{:?}", bytes);
    // println!(
    //     "First: {:?}, Second: {:?}",
    //     &bytes[..n / 2],
    //     &bytes[n / 2..]
    // );
    if bytes[..n / 2] == bytes[n / 2..] {
        return true;
    }
    false
}

fn part1(content: &str) {
    let first_line = content.lines().next().unwrap();
    let mut acc = 0;
    for range in first_line.split(",") {
        let nums: Vec<u64> = range
            .split('-')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        if nums.len() != 2 {
            panic!("Erreur de parsing");
        }
        let (nb1, nb2) = (nums[0], nums[1]);
        for n in nb1..=nb2 {
            if is_twice_numbers(n) {
                acc += n;
            }
        }
    }
    println!("{acc}")
}

fn is_at_least_twice_numbers(n: u64) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes(); // [1, 1]
    let n = bytes.len(); // 2
    for size in 1..=n / 2 {
        // size = 1
        if !n.is_multiple_of(size) {
            continue;
        }
        let mut all_equal = true;
        for i in 0..(n / size) - 1 {
            if bytes[i * size..(i + 1) * size] != bytes[(i + 1) * size..(i + 2) * size] {
                all_equal = false;
                break;
            }
        }
        if all_equal {
            return true;
        }
    }
    false
}

fn part2(content: &str) {
    let first_line = content.lines().next().unwrap();
    let mut acc = 0;
    for range in first_line.split(",") {
        let nums: Vec<u64> = range
            .split('-')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        if nums.len() != 2 {
            panic!("Erreur de parsing");
        }
        let (nb1, nb2) = (nums[0], nums[1]);
        for n in nb1..=nb2 {
            if is_at_least_twice_numbers(n) {
                // println!("{n}");
                acc += n;
            }
        }
    }
    println!("{acc}")
}

fn main() -> std::io::Result<()> {
    let file = File::open("input").expect("Impossible d'ouvrir le fichier");
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    println!("Part 1 : ");
    part1(&content);
    println!("Part 2 : ");
    part2(&content);
    Ok(())
}
