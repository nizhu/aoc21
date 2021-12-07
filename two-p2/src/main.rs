use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let filename = "./input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut h = 0;
  let mut d = 0;
  let mut a = 0;

  for (_index, line) in reader.lines().enumerate() {
    let l = line.unwrap();
    let split: Vec<&str> = l.split(' ').collect();
    let (word, num) = (split[0], split[1].parse::<i32>().unwrap());
    
    if word == "forward" {
      h = h + num;
      d = d + a * num;
    } else if word == "down" {
      a = a + num;
    } else if word == "up" {
      a = a - num;
    }
  }

  println!("{}", h * d);
}
