use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let filename = "./input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut count = 0;
  let mut cur = i32::MAX;

  for (index, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    let num:i32 = line.trim().parse().unwrap();
    if num > cur {
      count = count + 1;
    }

    cur = num;
  }

  println!("{}", count);
}
