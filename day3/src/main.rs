use std::fs::File;
use std::io::{BufReader, Read};

fn part1(content: &str) {
    let mut acc = 0;
    for line in content.lines() {
        let nb_vec = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        let max_vec = nb_vec.iter().max().expect("no max");
        let index_max = nb_vec
            .iter()
            .position(|&x| x == *max_vec)
            .expect("max not found");
        // println!("index_max: {index_max}, len: {}", nb_vec.len());
        if index_max == nb_vec.len() - 1 {
            let second_max = max_vec;
            let max_vec = nb_vec[..index_max].iter().max().expect("no second max");
            let max_joltage = max_vec * 10 + second_max;
            acc += max_joltage;
        } else {
            let second_max = nb_vec[index_max + 1..].iter().max().expect("no second max");
            let max_joltage = max_vec * 10 + second_max;
            acc += max_joltage;
        }
    }
    println!("{acc}");
}

fn joltage(number: Vec<u64>, budget_left: u64) -> u64 {
    if budget_left == 0 {
        number.iter().max().unwrap().to_owned()
    } else {
        let end = number.len().saturating_sub(budget_left as usize);
        let d = number[..end].iter().max().unwrap().to_owned();
        let d_index = number.iter().position(|&x| x == d).unwrap();
        d * 10u64.pow(budget_left as u32) + joltage(number[d_index + 1..].into(), budget_left - 1)
    }
}

fn part2(content: &str) {
    // DP
    let mut acc = 0;
    for line in content.lines() {
        let mj = joltage(
            line.chars()
                .map(|c| c.to_digit(10).unwrap().into())
                .collect(),
            11,
        );
        acc += mj;
        // println!("max_joltage for line {}: {}", line, mj);
    }
    println!("{acc}");
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
