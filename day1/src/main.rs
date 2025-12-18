use std::fs::File;
use std::io::{BufReader, Read};

fn part1(content: &str) {
    let mut acc = 50;
    let mut counter_at_zero = 0;
    for line in content.lines() {
        // parse the line as (R+L)+(integer 0<=x<100)
        let first_char = line.chars().next().unwrap();
        let mut number: i32 = line[1..]
            .parse()
            .expect("Erreur lors de la conversion en entier");
        number = match first_char {
            'R' => number,
            'L' => -number,
            _ => panic!("Instruction inconnue: {}", line),
        };
        acc += number;
        acc %= 100;
        if acc == 0 {
            counter_at_zero += 1;
        }
    }
    println!("Accumulator : {acc}");
    println!("Nb of 0s: {counter_at_zero}");
}

fn part2(content: &str) {
    let mut acc = 50;
    let mut counter_at_zero = 0;
    for line in content.lines() {
        // parse the line as (R+L)+(integer 0<=x<100)
        let first_char = line.chars().next().unwrap();
        let mut number: i32 = line[1..]
            .parse()
            .expect("Erreur lors de la conversion en entier");
        if first_char == 'L' {
            number = -number;
        }
        let latest = acc;
        acc += number;
        let mut crossings = (acc.div_euclid(100) - latest.div_euclid(100)).abs();
        if first_char == 'L' {
            if latest == 0 {
                crossings -= 1
            }
            if acc.rem_euclid(100) == 0 {
                crossings += 1;
            }
        }
        counter_at_zero += crossings;
        acc = acc.rem_euclid(100);
    }
    println!("Accumulator : {acc}");
    println!("Nb of 0s: {counter_at_zero}");
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt").expect("Impossible d'ouvrir le fichier");
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    println!("Part 1 : ");
    part1(&content);
    println!("Part 2 : ");
    part2(&content);
    Ok(())
}
