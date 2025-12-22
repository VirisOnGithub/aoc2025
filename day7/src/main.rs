use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufReader, Read};

fn part1(content: &str) {
    // Transpose the grid so that columns become rows and rows become columns
    let lines: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let grid: Vec<Vec<char>> = if lines.is_empty() {
        Vec::new()
    } else {
        (0..lines[0].len())
            .map(|col| lines.iter().map(|row| row[col]).collect())
            .collect()
    };
    let grid_height = grid.len();
    let grid_length = grid[0].len();
    // println!("Grid dimensions: {} x {}", grid_height, grid_length);
    let mut count = 0;
    // locate the source
    let source_location = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'S').map(|j| (i, j)))
        .expect("Source not found");
    // println!("Source located at: {:?}", source_location);
    let mut queue = VecDeque::new();
    queue.push_back(source_location);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((x, y)) = queue.pop_front() {
        // println!("Visiting: ({}, {})", x, y);
        if visited.contains(&(x, y)) {
            continue;
        }

        let ch = grid[x][y];
        if ch == 'S' || ch == '.' {
            if y + 1 < grid_length {
                queue.push_back((x, y + 1));
            }
            continue;
        }

        // println!("Le caractère trouvé est {}", ch);

        visited.insert((x, y));

        let mut split = false;

        for dir in [-1_isize, 1_isize] {
            let new_x = x as isize + dir;
            if new_x < 0 || (new_x as usize) >= grid_height {
                continue;
            }

            split = true;
            queue.push_back((new_x as usize, y));
        }

        if split {
            count += 1;
        }
    }

    println!("Number of splits: {}", count);
}

fn part2(content: &str) {
    let mut lines = content.lines();
    let first_line = lines.next().unwrap();
    let start = first_line.find('S').expect("Source not found");

    let mut timelines: Vec<usize> = Vec::new();
    timelines.resize(first_line.len(), 1);

    for line in lines
        .rev()
        .enumerate()
        .filter_map(|(i, l)| (i % 2 != 0).then_some(l))
    {
        for (j, ch) in line.chars().enumerate() {
            if ch == '^' {
                timelines[j] = timelines[j + 1] + timelines[j - 1]
            }
        }
    }

    println!("Number of timelines: {}", timelines[start]);
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
