use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut count: u32 = 0;

    for line in reader.lines() {
        let card = line.unwrap();

        let (numbers, winning_numbers) = card.split_once(" | ").unwrap();
        let numbers = numbers.split_once(": ").unwrap().1;

        let set_numbers: HashSet<u8> = numbers
            .split(' ')
            .filter_map(|x| x.parse::<u8>().ok())
            .collect();

        let set_winning_numbers: HashSet<u8> = winning_numbers
            .split(' ')
            .filter_map(|x| x.parse::<u8>().ok())
            .collect();

        let intersection_count: u32 = set_numbers
            .intersection(&set_winning_numbers)
            .count()
            .try_into()
            .unwrap();

        if intersection_count > 0 {
            let points = u32::pow(2, intersection_count - 1);

            count += points;
        }
    }
    println!("{:?}", count);
}
