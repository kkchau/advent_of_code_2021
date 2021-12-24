use std::io::{BufRead, BufReader};
use std::path::Path;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let display = path.display();

    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("{}: {}", why, display)
    };

    let reader = BufReader::new(file);
    let inputs = reader.lines();

    let mut aim: i32 = 0;
    let mut position: i32 = 0;
    let mut depth: i32 = 0;

    for input in inputs {
        let val = input.unwrap();
        let instructions: Vec<&str> = val.split(" ").collect();
        let direction = instructions[0];
        let distance: i32 = match instructions[1].trim().parse() {
            Ok(value) => value,
            Err(reason) => panic!("{}", reason)
        };
        println!("{}----{}", direction, distance);

        if direction == "forward" {
            position += distance;
            depth += aim * distance;
        } else if direction == "up" {
            aim -= distance;
        } else if direction == "down" {
            aim += distance;
        }
    }

    println!("{}", position * depth);
}
