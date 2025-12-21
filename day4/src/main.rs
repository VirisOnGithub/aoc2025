use std::fs::File;
use std::io::{BufReader, Read};

fn is_accessible(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let mut nb_rolls = 0;
    for (di, dj) in directions.iter() {
        let ni = i as isize + di;
        let nj = j as isize + dj;
        if ni >= 0
            && ni < grid.len() as isize
            && nj >= 0
            && nj < grid[0].len() as isize
            && grid[ni as usize][nj as usize] == '@'
        {
            nb_rolls += 1;
        }
    }
    nb_rolls < 4
}

fn part1(content: &str) {
    let mut accessible_rolls = 0;
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' && is_accessible(&grid, i, j) {
                accessible_rolls += 1;
                print!("x");
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!();
    }
    println!("Accessible rolls: {}", accessible_rolls);
}

fn part2(content: &str) {
    let mut accessible_rolls = 0;
    let mut grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let mut has_moved_last_iter = true;
    while has_moved_last_iter {
        has_moved_last_iter = false;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '@' && is_accessible(&grid, i, j) {
                    accessible_rolls += 1;
                    has_moved_last_iter = true;
                    grid[i][j] = '.';
                    print!("x");
                } else {
                    print!("{}", grid[i][j]);
                }
            }
            println!();
        }
        println!("\n")
    }
    println!("Accessible rolls: {}", accessible_rolls);
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
