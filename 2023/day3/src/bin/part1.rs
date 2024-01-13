use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
struct Point {
    col: usize,
    row: usize,
    value: char,
}

impl PartialEq<AdjacentPoint> for Point {
    fn eq(&self, other: &AdjacentPoint) -> bool {
        return self.col == other.col && self.row == other.row;
    }
}

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
struct AdjacentPoint {
    col: usize,
    row: usize,
}

impl PartialEq<Point> for AdjacentPoint {
    fn eq(&self, other: &Point) -> bool {
        return self.col == other.col && self.row == other.row;
    }
}

impl From<Point> for AdjacentPoint {
    fn from(p: Point) -> AdjacentPoint {
        return AdjacentPoint {
            col: p.col,
            row: p.row,
        };
    }
}

fn main() {
    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum: usize = 0;

    let mut row: usize = 0;
    let mut symbols_coordinates: HashSet<Point> = HashSet::new();
    let mut numbers_coordinates: HashMap<Vec<Point>, String> = HashMap::new();

    for line in reader.lines() {
        let l = line.unwrap();

        let points = l.chars().enumerate().map(|(col, value)| {
            return Point { col, row, value };
        });

        let line_length = points.clone().count();

        let mut last_number: String = String::new();
        let mut coordinates: Vec<Point> = vec![];

        for point in points {
            // build number
            if point.value.is_digit(10) {
                last_number.push(point.value);
                coordinates.push(point);

                if is_last_column(point.col, line_length) {
                    if !is_adjacent_to_symbol(&mut coordinates, &symbols_coordinates, line_length) {
                        numbers_coordinates.insert(coordinates, last_number);
                    } else {
                        sum += last_number.parse::<usize>().unwrap();
                    }

                    last_number = String::new();
                    coordinates = vec![];
                }
                continue;
            }

            // end of number
            if !last_number.is_empty() {
                if !is_adjacent_to_symbol(&mut coordinates, &symbols_coordinates, line_length) {
                    numbers_coordinates.insert(coordinates, last_number);
                } else {
                    sum += last_number.parse::<usize>().unwrap();
                }

                last_number = String::new();
                coordinates = vec![];
            }

            if is_symbol(point.value) {
                symbols_coordinates.insert(point);
                let adjacency_set = get_adjacency_set(point, line_length);

                let sum_of_adjacent_numbers: usize = numbers_coordinates
                    .clone()
                    .into_iter()
                    .filter(|(coords, _)| in_current_or_last_row(coords, point))
                    .filter(|(coords, _)| {
                        // TODO this can be abstracted into is_adjacent_to_symbol
                        return coords
                            .iter()
                            .any(|&p| adjacency_set.contains(&AdjacentPoint::from(p)));
                    })
                    .map(|(coords, n)| {
                        let num = n.parse::<usize>().unwrap();
                        numbers_coordinates.remove(&coords);
                        return num;
                    })
                    .sum();
                sum += sum_of_adjacent_numbers;
            }
        }

        row += 1;
    }

    println!("{:?}", sum);
}

fn is_symbol(c: char) -> bool {
    return !c.is_digit(10) && c != '.';
}

// TODO cleanup this mess
fn get_adjacency_set(point: Point, num_of_columns: usize) -> HashSet<AdjacentPoint> {
    let mut adjacency_set: HashSet<AdjacentPoint> = HashSet::new();

    let max_column = num_of_columns - 1;

    // first char in input
    if point.col == 0 && point.row == 0 {
        return adjacency_set;
    }

    // not first col in input
    if point.col != 0 {
        let left = AdjacentPoint {
            col: point.col - 1,
            row: point.row,
        };
        adjacency_set.insert(left);

        // not last row in input
        if point.row != 0 {
            let top_left = AdjacentPoint {
                col: point.col - 1,
                row: point.row - 1,
            };
            adjacency_set.insert(top_left);
        }
    }

    // not first row in input
    if point.row != 0 {
        let top = AdjacentPoint {
            col: point.col,
            row: point.row - 1,
        };
        adjacency_set.insert(top);

        // not last col in input
        if point.col != max_column {
            let top_right = AdjacentPoint {
                col: point.col + 1,
                row: point.row - 1,
            };
            adjacency_set.insert(top_right);
        }
    }

    return adjacency_set;
}

fn is_adjacent_to_symbol(
    coordinates: &mut Vec<Point>,
    symbols: &HashSet<Point>,
    num_of_columns: usize,
) -> bool {
    let adjacency_set: Vec<AdjacentPoint> = coordinates
        .iter()
        .flat_map(|&p| get_adjacency_set(p, num_of_columns))
        .collect();

    return symbols
        .iter()
        .any(|&p| adjacency_set.contains(&AdjacentPoint::from(p)));
}

fn in_current_or_last_row(coords: &Vec<Point>, point: Point) -> bool {
    let row = coords.first().unwrap().row;
    let is_valid_row = row as isize - point.row as isize <= 1;

    return is_valid_row;
}

fn is_last_column(col: usize, line_length: usize) -> bool {
    return col == line_length - 1;
}
