use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = match File::open(Path::new(&args[1])) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let reader = BufReader::new(file);

    let mut bit_freq: Vec<i64> = Vec::new();

    for input in reader.lines() {
        let bit_string = match input {
            Ok(value) => value,
            Err(why) => panic!("{}", why),
        };
        let bit_string: Vec<char> = bit_string.chars().collect();
        for (index, bit) in bit_string.iter().enumerate() {
            let bit: &u32 = &char::to_digit(*bit, 10).unwrap();

            if bit_freq.len() <= index {
                bit_freq.resize(index + 1, 0)
            };

            match bit {
                0 => bit_freq[index] -= 1,
                1 => bit_freq[index] += 1,
                _ => panic!("Can't parse bit {}", bit),
            };
        }
    }

    let bits_length = bit_freq.len();

    let mut gamma_bits = vec![0; bits_length];
    let mut epsilon_bits = vec![0; bits_length];
    for (index, bit) in bit_freq.iter().enumerate() {
        if bit > &0 {
            gamma_bits[index] = 1;
            epsilon_bits[index] = 0;
        } else if bit < &0 {
            gamma_bits[index] = 0;
            epsilon_bits[index] = 1;
        }
    }

    let gamma_bit_string: String = gamma_bits.iter().map(|&s| s.to_string()).collect();
    let gamma_rate = isize::from_str_radix(&gamma_bit_string, 2).unwrap();
    let epsilon_bit_string: String = epsilon_bits.iter().map(|&s| s.to_string()).collect();
    let epsilon_rate = isize::from_str_radix(&epsilon_bit_string, 2).unwrap();

    let answer = gamma_rate * epsilon_rate;
    println!("{}", answer);
}
