use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn mod100(x: i32) -> i32 {
    ((x % 100) + 100) % 100
}

fn main() -> io::Result<()> {
    let path = "src/inputs/part1.txt";

    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut res = 0;
    let mut pos = 50;


    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line);
        let direction = line.chars().next().unwrap();
        let rest = &line[direction.len_utf8()..];
        let num: i32 = rest.parse().expect("Invalid number");        
        
        let full_rotations = num / 100;
        let remainder = mod100(num);

        res += full_rotations;

        if direction == 'R' {
            if pos + remainder >= 100 {
                res += 1;
            }
            pos = mod100(pos + remainder);
        } else {
            if pos != 0 && pos - remainder <= 0 {
                res += 1;
            }
            pos = mod100(pos - remainder);
        }
    }

    println!("{}", res);

    Ok(())
}