use std:: io::{ BufRead, BufReader};
use std::fs::File;

fn main() {
    let digit_strings = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];
    let mut sum = 0;

    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap();
        let first_digit = l
                            .chars()
                            .find(|x| is_digit(x));
                            .unwrap()
                            .to_digit(10)
                            .unwrap();


        let second_digit = l
                            .chars()
                            .rev()
                            .find(|x| is_digit(x));
                            .unwrap()
                            .to_digit(10)
                            .unwrap();


        let concat_digits = (first_digit * 10) + second_digit;

        sum += concat_digits;

    }

    println!("Sum of numbers: {}", sum);
}


fn is_digit() -> bool {

}