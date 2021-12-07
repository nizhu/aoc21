use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let filename = "./input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut f = 0;
  let mut u = 0;

  for (_index, line) in reader.lines().enumerate() {
    let l = line.unwrap();
    let split: Vec<&str> = l.split(' ').collect();
    let (word, num) = (split[0], split[1].parse::<i32>().unwrap());
    
    if word == "forward" {
      f = f + num
    } else if word == "down" {
      u = u + num
    } else if word == "up" {
      u = u - num
    } else {
      f = f - num
    }
  }

  println!("{}", f * u);
}
