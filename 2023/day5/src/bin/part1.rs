use std::{fs::read_to_string, vec};

// TODO create struct to hold range values for each map
// {dest_start, dest_end, source_start, source_end}

// TODO convert everything to integers

fn main() {
    let lines = read_lines("input.txt");

    let mut seeds: Vec<String> = lines[0]
        .split(' ')
        .map(String::from)
        .collect::<Vec<String>>()[1..]
        .into();

    // TODO remove multiple collects
    let maps: Vec<Vec<Vec<Vec<usize>>>> = lines[2..]
        .iter()
        .filter(|&line| !line.contains("map"))
        .map(|map| get_map_ranges(map))
        .collect::<Vec<Vec<Vec<usize>>>>()
        .split(|m| m.is_empty())
        .map(Vec::from)
        .collect();

    // TODO move to iterator above
    for map in maps {
        seeds = seeds.iter().map(|seed| map_value(seed, &map)).collect();
    }

    println!("{:?}", seeds);

    let min_seed = seeds
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .min()
        .unwrap();
    println!("{:?}", min_seed);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

fn get_map_ranges(map: &String) -> Vec<Vec<usize>> {
    if map == "" {
        return vec![];
    }
    let range_values: Vec<usize> = map
        .split(' ')
        .filter_map(|f| f.parse::<usize>().ok())
        .collect();

    let r1 = range_values[0];
    let r2 = range_values[1];
    let length = range_values[2];

    let into_range = vec![r1, r1 + length - 1];
    let from_range = vec![r2, r2 + length - 1];

    return vec![from_range, into_range];
}

fn map_value(seed: &String, map: &Vec<Vec<Vec<usize>>>) -> String {
    for range in map {
        let seed_int = seed.parse::<usize>().unwrap();
        let source_range = &range[0];
        let destination_range = &range[1];
        if seed_int >= source_range[0] && seed_int <= source_range[1] {
            return (seed_int - source_range[0] + destination_range[0]).to_string();
        }
    }
    return seed.into();
}
