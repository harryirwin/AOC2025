use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn has_repeating_sequence(s: &str) -> bool {
    let len = s.len();

    for seq_len in 1..=len / 2 {
        if len % seq_len != 0 {
            continue;
        }

        let repeat_count = len / seq_len;
        let seq = &s[..seq_len];

        if seq.repeat(repeat_count) == s {
            return true;
        }
    }

    false
}


fn main() -> io::Result<()> {
    let path = "src/inputs/part1.txt";

    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut res = 0;

    if let Some(Ok(line)) = reader.lines().next() {
        let ranges: Vec<&str> = line.split(',').collect();

        for range in ranges {
            if let Some((start_str, end_str)) = range.split_once('-') {
                let start: u64 = start_str.parse().unwrap();
                let end: u64 = end_str.parse().unwrap();
                for i in start..=end {
                    let s = i.to_string();

                    if has_repeating_sequence(&s) {
                        res += i;
                    }
                }
            }
        }
    }
    println!("{}", res);
    Ok(())
}
