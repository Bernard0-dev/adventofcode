use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const MAX_RED_CUBES: u32 = 12;
    const MAX_GREEN_CUBES: u32 = 13;
    const MAX_BLUE_CUBES: u32 = 14;
    let maxs: Vec<u32> = vec![MAX_RED_CUBES, MAX_GREEN_CUBES, MAX_BLUE_CUBES];

    let max_cubes: &u32 = &maxs.iter().max().unwrap();

    const RED_STRING: &str = "red";
    const GREEN_STRING: &str = "green";
    const BLUE_STRING: &str = "blue";
    let colors: Vec<&str> = vec![RED_STRING, GREEN_STRING, BLUE_STRING];

    let zip_cube = colors.iter().zip(maxs.iter());
    // TODO check a better way to do this without &&
    let map_color_to_max = HashMap::<&&str, &u32>::from_iter(zip_cube);

    let mut sum_possible_sets = 0;
    // let mut possible_sets: Vec<String> = vec![];

    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap();

        let line_header = &l.split(':').next().unwrap();

        let line_num = line_header
            .split(' ')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let idx_start_of_sets = line_header.len() + 2;
        let sets = &l[idx_start_of_sets..];
        // println!("{:?}", sets);

        let cube_colors = sets
            .split([' ', ';', ','])
            .filter(|&s| s.contains(char::is_alphabetic))
            .collect::<Vec<&str>>();

        // println!("{:?}", cube_colors);

        let cube_nums: Vec<u32> = sets
            .split(' ')
            .filter(|&s| s.contains(char::is_numeric))
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        // println!("{:?}", cube_nums);

        // there's 1 value that's bigger than the number of any cube
        if cube_nums.iter().filter(|&x| x > max_cubes).count() > 0 {
            continue;
        }

        let zip_cube = cube_colors.iter().zip(cube_nums.iter());
        let mut flag_impossible_set = false;
        for result in zip_cube {
            let (cube_color, cube_num) = result;
            // check if cube_num is over the max number of cubes of X color
            if cube_num > map_color_to_max.get(cube_color).unwrap() {
                flag_impossible_set = true;
                break;
            }
        }

        if flag_impossible_set {
            continue;
        }

        sum_possible_sets += line_num;

        // possible_sets.push(l)
    }

    println!("{}", sum_possible_sets);
    // for s in possible_sets {
    //     println!("{}", s);
    // }
}
