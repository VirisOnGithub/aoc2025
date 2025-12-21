use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};

type INT = u64;

fn part1(content: &str) {
    let mut range_part = true;
    let mut all_ranges: Vec<(INT, INT)> = Vec::new();
    let mut fresh_products = 0;
    'lines: for line in content.lines() {
        if line.is_empty() {
            range_part = false;
            continue;
        }
        if range_part {
            let parts: Vec<INT> = line.split('-').map(|r| r.parse::<INT>().unwrap()).collect();
            if parts.len() == 2 {
                let range = (parts[0], parts[1]);
                all_ranges.push(range);
            }
        } else {
            let product_id: INT = line.parse().unwrap();
            for range in &all_ranges {
                if product_id >= range.0 && product_id <= range.1 {
                    fresh_products += 1;
                    // println!(
                    //     "This product is fresh ({product_id}) because it belongs to the range {}-{}",
                    //     range.0, range.1
                    // );
                    continue 'lines;
                }
            }
        }
    }
    println!("There are {fresh_products} fresh products");
}

fn part2(content: &str) {
    let mut ranges: Vec<(INT, INT)> = Vec::new();

    for line in content.lines() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('-').map(|r| r.parse::<INT>().unwrap());
        if let (Some(start), Some(end)) = (parts.next(), parts.next()) {
            ranges.push((start, end));
        }
    }

    // Sort by start
    ranges.sort_unstable_by_key(|r| r.0);

    // Merge overlapping ranges and count
    let count: INT = ranges
        .iter()
        .fold((0, INT::MIN), |(total, prev_end), &(start, end)| {
            if start > prev_end {
                (total + (end - start + 1), end)
            } else if end > prev_end {
                (total + (end - prev_end), end)
            } else {
                (total, prev_end)
            }
        })
        .0;

    println!("There are {} fresh products", count);
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
