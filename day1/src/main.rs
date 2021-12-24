use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();

    let window_size: usize = match args[2].parse() {
        Ok(value) => value,
        Err(why) => panic!("{}: {}", why, &args[2])
    };
    let mut pre_window = Vec::new();

    let path = Path::new(&args[1]);
    let display = path.display();

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open file {}: {}", &display, &why),
    };

    let reader = BufReader::new(file);
    let inputs = reader.lines();

    let mut increasing: i32 = 0;

    for input in inputs {
        let val = input.unwrap();
        let cur_value: u32 = match val.parse() {
            Ok(value) => value,
            Err(why) => panic!("{}: {}", why, &val)
        };

        if pre_window.len() < window_size {
            pre_window.push(cur_value);
            continue;
        } else {
            let mut cur_window = Vec::new();
            cur_window.extend(&pre_window[1..]);
            cur_window.push(cur_value);

            let pre_window_sum: u32 = pre_window.iter().sum();
            let cur_window_sum: u32 = cur_window.iter().sum();

            if pre_window_sum < cur_window_sum {
                increasing += 1;
            }

            pre_window = cur_window;
        }
    };
    println!("{}", increasing);
}
