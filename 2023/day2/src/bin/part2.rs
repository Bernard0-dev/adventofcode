use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const RED_STRING: &str = "red";
    const GREEN_STRING: &str = "green";
    const BLUE_STRING: &str = "blue";

    let mut sum_of_powers = 0;

    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap();

        let idx_start_of_sets = &l.find(':').unwrap() + 2;
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

        let mut min_number_of_cubes =
            HashMap::<&str, u32>::from([(RED_STRING, 0), (GREEN_STRING, 0), (BLUE_STRING, 0)]);

        let zip_cube = cube_colors.iter().zip(cube_nums.iter());
        for result in zip_cube {
            let (cube_color, &cube_num) = result;
            min_number_of_cubes.insert(
                cube_color,
                cmp::max(min_number_of_cubes[cube_color], cube_num),
            );
        }

        let power: u32 = min_number_of_cubes.values().product();

        sum_of_powers += power;
    }

    println!("{}", sum_of_powers);
}
