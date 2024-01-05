use std::{fs::read_to_string, vec};

#[derive(Debug, Copy, Clone, Default)]
struct Map {
    source_start: usize,
    source_end: usize,
    dest_start: usize,
    dest_end: usize,
}

impl Map {
    pub fn is_empty(&self) -> bool {
        return self.source_start == 0
            && self.source_end == 0
            && self.dest_start == 0
            && self.dest_end == 0;
    }
}

// TODO convert everything to integers

fn main() {
    let lines = read_lines("input.txt");

    let mut seeds: Vec<String> = lines[0]
        .split(' ')
        .map(String::from)
        .collect::<Vec<String>>()[1..]
        .into();

    // TODO remove multiple collects
    let maps: Vec<Vec<Map>> = lines[2..]
        .iter()
        .filter(|&line| !line.contains("map"))
        .map(|map| get_map_ranges(map))
        .collect::<Vec<Map>>()
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

fn get_map_ranges(map: &String) -> Map {
    if map == "" {
        return Map::default();
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

    println!("{:?}", into_range);
    println!("{:?}", from_range);

    let map = Map {
        source_start: from_range[0],
        source_end: from_range[1],
        dest_start: into_range[0],
        dest_end: into_range[1],
    };

    return map;
}

fn map_value(seed: &String, maps: &Vec<Map>) -> String {
    for map in maps {
        let seed_int = seed.parse::<usize>().unwrap();

        if seed_int >= map.source_start && seed_int <= map.source_end {
            return (seed_int - map.source_start + map.dest_start).to_string();
        }
    }
    return seed.into();
}
