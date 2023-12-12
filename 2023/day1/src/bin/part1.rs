use std:: io::{ BufRead, BufReader};
use std::fs::File;

fn main() {
    let mut sum = 0;

    let filename = "./test-input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap();
        let first_digit = l
                            .chars()
                            .find(|x| x.is_digit(10))
                            .unwrap()
                            .to_digit(10)
                            .unwrap();


        let second_digit = l
                            .chars()
                            .rev()
                            .find(|x| x.is_digit(10))
                            .unwrap()
                            .to_digit(10)
                            .unwrap();

        let concat_digits = (first_digit * 10) + second_digit;

        sum += concat_digits;

    }

    println!("Sum of numbers: {}", sum);
}
