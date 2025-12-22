use std::fs::File;
use std::io::{BufReader, Read};

fn spaces_parsed(string: &str) -> Vec<&str> {
    string.split_whitespace().collect()
}

fn part1(content: &str) {
    let mut lines = content.lines().collect::<Vec<_>>();
    let operator_line = lines.pop().expect("No operator line found");
    let acc: Vec<Vec<u64>> = lines
        .iter()
        .map(|line| {
            spaces_parsed(line)
                .iter()
                .map(|n| n.parse::<u64>().expect("Failed to parse number"))
                .collect()
        })
        .collect();

    let mut res = 0u64;
    for (index, operator) in spaces_parsed(operator_line).iter().enumerate() {
        let nth_column_numbers: Vec<u64> = acc.iter().map(|r| r[index]).collect();
        res += match *operator {
            "*" => nth_column_numbers.iter().copied().product::<u64>(),
            "+" => nth_column_numbers.iter().copied().sum::<u64>(),
            _ => panic!("Operator not covered: `{}`", operator),
        }
    }
    println!("{res}");
}

fn determine_char_lengths(last_line: &str) {
    let mut acc: Vec<u64> = Vec::new();
    let mut space_counter;
    if last_line.starts_with(" ") {
        panic!("There was a space, whereas there shouldn't be");
    } else {
        let mut chars = last_line.chars();
        while chars.next().is_some() {
            space_counter = 0;
            while let Some(c) = chars.next()
                && c == ' '
            {
                space_counter += 1;
            }
            acc.push(space_counter + 1);
        }
    }
    println!("{:?}", acc);
}

fn part2(content: &str) {
    let mut lines = content.lines().collect::<Vec<_>>();
    determine_char_lengths(lines[lines.len() - 1]);
}

fn main() -> std::io::Result<()> {
    let file = File::open("testinput").expect("Impossible d'ouvrir le fichier");
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    println!("Part 1 : ");
    part1(&content);
    println!("Part 2 : ");
    part2(&content);
    Ok(())
}
