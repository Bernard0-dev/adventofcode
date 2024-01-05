use std::{cmp, collections::HashSet, fs::read_to_string};

#[derive(Debug, Copy, Clone)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn main() {
    let lines = read_lines("input.txt");

    const GALAXY: char = '#';
    const EXPANSION_SIZE: usize = 1_000_000;

    let mut rows_without_galaxies: HashSet<usize> = HashSet::from_iter(0..lines.len());
    let mut cols_without_galaxies: HashSet<usize> = HashSet::from_iter(0..lines[0].len());

    let mut galaxies: Vec<Galaxy> = vec![];

    for (line_num, line) in lines.iter().enumerate() {
        for (col_num, c) in line.char_indices() {
            if c == GALAXY {
                let galaxy = Galaxy {
                    x: col_num,
                    y: line_num,
                };
                galaxies.push(galaxy);
                rows_without_galaxies.remove(&galaxy.y);
                cols_without_galaxies.remove(&galaxy.x);
            }
        }
    }

    let sum_distances: usize = galaxies
        .iter()
        .enumerate()
        .flat_map(|(idx, g1)| {
            galaxies.iter().skip(idx + 1).map(|g2| {
                calc_distance(
                    g1,
                    g2,
                    &rows_without_galaxies,
                    &cols_without_galaxies,
                    EXPANSION_SIZE,
                )
            })
        })
        .sum::<usize>();

    println!("{:?}", sum_distances);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

fn calc_distance(
    g1: &Galaxy,
    g2: &Galaxy,
    rows_without_galaxies: &HashSet<usize>,
    cols_without_galaxies: &HashSet<usize>,
    expansion_size: usize,
) -> usize {
    let min_x = cmp::min(g1.x, g2.x);
    let max_x = cmp::max(g1.x, g2.x);
    let min_y = cmp::min(g1.y, g2.y);
    let max_y = cmp::max(g1.y, g2.y);

    let expanded_rows = rows_without_galaxies
        .iter()
        .filter(|&&row| row > min_y && row < max_y)
        .count();

    let expanded_cols = cols_without_galaxies
        .iter()
        .filter(|&&col| col > min_x && col < max_x)
        .count();

    let x_distance = max_x - min_x;
    let y_distance = max_y - min_y;

    let expanded_distance = (expanded_rows + expanded_cols) * (expansion_size - 1);

    let distance = x_distance + y_distance + expanded_distance;

    return distance;
}
